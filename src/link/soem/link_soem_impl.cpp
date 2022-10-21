// File: autdsoem.cpp
// Project: autdsoem
// Created Date: 23/08/2019
// Author: Shun Suzuki
// -----
// Last Modified: 21/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2019-2020 Shun Suzuki. All rights reserved.
//

#include "link_soem_impl.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6285 6385 26437 26800 26498 26451 26495)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include <spdlog/fmt/fmt.h>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

#include <cstdint>
#include <memory>
#include <queue>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "./ethercat.h"
#include "autd3/link/soem.hpp"
#include "ecat_thread/ecat_thread.hpp"

namespace {
std::string lookup_autd() {
  spdlog::debug("looking for AUTD...");
  const auto* adapters = ec_find_adapters();
  for (const auto* adapter = adapters; adapter != nullptr; adapter = adapter->next) {
    if (ec_init(adapter->name) <= 0) continue;
    if (const auto wc = ec_config_init(0); wc <= 0) continue;
    if (std::strcmp(ec_slave[1].name, "AUTD") == 0) {
      spdlog::debug("AUTD found on {} ({})", adapter->name, adapter->desc);
      return {adapter->name};
    }
  }
  throw std::runtime_error("No AUTD3 devices found");
}
}  // namespace

namespace autd3::link {

bool SOEMLink::is_open() { return _is_open.load(); }

bool SOEMLink::send(const driver::TxDatagram& tx) {
  if (!_is_open.load()) throw std::runtime_error("link is closed");

  std::lock_guard lock(_send_mtx);
  _send_buf.push(tx.clone());
  return true;
}

bool SOEMLink::receive(driver::RxDatagram& rx) {
  if (!_is_open.load()) throw std::runtime_error("link is closed");
  rx.copy_from(_io_map.input());
  return true;
}

void SOEMLink::open(const core::Geometry& geometry) {
  if (is_open()) return;

  std::queue<driver::TxDatagram>().swap(_send_buf);
  const auto cycle_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _send_cycle;
  _is_open.store(true);
  std::atomic<int32_t> wkc;
  _ecat_thread = std::thread([this, &wkc, cycle_time] {
    ecat_run(this->_high_precision, &this->_is_open, &this->_is_running, &wkc, cycle_time, this->_send_mtx, this->_send_buf, this->_io_map);
  });

  const auto dev_num = geometry.num_devices();

  try {
    if (_ifname.empty()) _ifname = lookup_autd();
  } catch (std::runtime_error&) {
    _is_open.store(false);
    if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
    throw;
  }

  spdlog::debug("interface name: {}", _ifname);

  if (ec_init(_ifname.c_str()) <= 0) {
    _is_open.store(false);
    if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
    throw std::runtime_error(fmt::format("No socket connection on {}", _ifname));
  }

  const auto wc = ec_config_init(0);
  if (wc <= 0) {
    _is_open.store(false);
    if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
    throw std::runtime_error("No slaves found");
  }
  for (auto i = 1; i <= wc; i++)
    if (std::strcmp(ec_slave[i].name, "AUTD") != 0) {
      _is_open.store(false);
      if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
      throw std::runtime_error(fmt::format("Slave[{}] is not AUTD3", i));
    }

  if (static_cast<size_t>(wc) != dev_num) {
    _is_open.store(false);
    if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
    throw std::runtime_error(fmt::format("The number of slaves you configured: {}, but found: {}", dev_num, wc));
  }

  _user_data = std::make_unique<uint32_t[]>(1);
  _user_data[0] = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
  ecx_context.userdata = _user_data.get();
  spdlog::debug("Sync0 interval: {} [ns]", driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle);

  if (_sync_mode == SYNC_MODE::DC) {
    for (int cnt = 1; cnt <= ec_slavecount; cnt++)
      ec_slave[cnt].PO2SOconfigx = [](auto* context, auto slave) -> int {
        const auto cyc_time = static_cast<uint32_t*>(context->userdata)[0];
        ec_dcsync0(slave, true, cyc_time, 0U);
        return 0;
      };
    spdlog::debug("run mode: DC sync");
    spdlog::debug("Sync0 configured");
  }

  _io_map.resize(dev_num);
  ec_config_map(_io_map.get());

  ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE * 4);

  ec_configdc();

  ec_readstate();

  const auto expected_wkc = ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC;
  spdlog::debug("Calculated workcounter {}", expected_wkc);

  ec_slave[0].state = EC_STATE_OPERATIONAL;
  ec_writestate(0);

  _is_running = true;

  spdlog::debug("send interval: {} [ns]", cycle_time);

  std::this_thread::sleep_for(std::chrono::milliseconds(100));

  ec_statecheck(0, EC_STATE_OPERATIONAL, EC_TIMEOUTSTATE * 5);

  if (ec_slave[0].state != EC_STATE_OPERATIONAL) {
    _is_open.store(false);
    _is_running = false;
    if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
    if (this->_ecat_check_thread.joinable()) this->_ecat_check_thread.join();
    throw std::runtime_error("One ore more slaves are not responding");
  }

  if (_sync_mode == SYNC_MODE::FREE_RUN) {
    for (int slave = 1; slave <= ec_slavecount; slave++)
      ec_dcsync0(static_cast<uint16_t>(slave), true, driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle, 0U);
    spdlog::debug("run mode: Free Run");
    spdlog::debug("Sync0 configured");
  }

  _ecat_check_thread = std::thread([this, &wkc, expected_wkc] {
    while (this->_is_open.load()) {
      if ((this->_is_running && (wkc.load() < expected_wkc)) || ec_group[0].docheckstate) error_handle(&this->_is_open, this->_on_lost);
      std::this_thread::sleep_for(std::chrono::microseconds(100000));
    }
  });
}

void SOEMLink::close() {
  if (!is_open()) return;

  while (!_send_buf.empty()) std::this_thread::sleep_for(std::chrono::milliseconds(1));

  _is_open.store(false);
  _is_running = false;
  if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
  if (this->_ecat_check_thread.joinable()) this->_ecat_check_thread.join();

  const auto cyc_time = static_cast<uint32_t*>(ecx_context.userdata)[0];
  for (uint16_t slave = 1; slave <= static_cast<uint16_t>(ec_slavecount); slave++) ec_dcsync0(slave, false, cyc_time, 0U);

  ec_slave[0].state = EC_STATE_SAFE_OP;
  ec_writestate(0);
  ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE);

  ec_slave[0].state = EC_STATE_PRE_OP;
  ec_writestate(0);
  ec_statecheck(0, EC_STATE_PRE_OP, EC_TIMEOUTSTATE);

  ec_close();
}

SOEMLink::~SOEMLink() {
  try {
    this->close();
  } catch (std::exception&) {
  }
}

core::LinkPtr SOEM::build() {
  return std::make_unique<SOEMLink>(_high_precision, _ifname, _sync0_cycle, _send_cycle, std::move(_callback), _sync_mode);
}

std::vector<EtherCATAdapter> SOEM::enumerate_adapters() {
  auto* adapter = ec_find_adapters();
  std::vector<EtherCATAdapter> adapters;
  while (adapter != nullptr) {
    EtherCATAdapter info(std::string(adapter->desc), std::string(adapter->name));
    adapters.emplace_back(info);
    adapter = adapter->next;
  }
  return adapters;
}
}  // namespace autd3::link
