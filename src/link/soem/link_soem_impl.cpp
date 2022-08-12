// File: autdsoem.cpp
// Project: autdsoem
// Created Date: 23/08/2019
// Author: Shun Suzuki
// -----
// Last Modified: 12/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2019-2020 Shun Suzuki. All rights reserved.
//

#include "link_soem_impl.hpp"

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
  pcap_if_t* alldevs = nullptr;
  static char errbuf[PCAP_ERRBUF_SIZE] = {0};
  if (pcap_findalldevs(&alldevs, errbuf) == -1) {
    std::stringstream ss;
    ss << "failed to find devices: " << errbuf;
    throw std::runtime_error(ss.str());
  }

  for (const pcap_if_t* cursor = alldevs; cursor != nullptr; cursor = cursor->next) {
    if (ec_init(cursor->name) <= 0) continue;
    const auto wc = ec_config_init(0);
    if (wc <= 0) continue;
    if (std::strcmp(ec_slave[1].name, "AUTD") == 0) return std::string(cursor->name);
  }

  std::stringstream ss;
  ss << "No AUTD device is found.";
  throw std::runtime_error(ss.str());
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
  const auto dev_num = geometry.num_devices();

  if (_ifname.empty()) _ifname = lookup_autd();

  if (ec_init(_ifname.c_str()) <= 0) {
    std::stringstream ss;
    ss << "No socket connection on " << _ifname;
    throw std::runtime_error(ss.str());
  }

  const auto wc = ec_config_init(0);
  if (wc <= 0) throw std::runtime_error("No slaves found!");

  for (auto i = 1; i <= wc; i++)
    if (std::strcmp(ec_slave[i].name, "AUTD") != 0) {
      std::stringstream ss;
      ss << "Slave[" << i << "] is not AUTD.";
      throw std::runtime_error(ss.str());
    }

  if (static_cast<size_t>(wc) != dev_num) {
    std::stringstream ss;
    ss << "The number of slaves you configured: " << dev_num << ", but found: " << wc;
    throw std::runtime_error(ss.str());
  }

  _user_data = std::make_unique<uint32_t[]>(1);
  _user_data[0] = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
  ecx_context.userdata = _user_data.get();
  auto dc_config = [](ecx_contextt* const context, const uint16_t slave) -> int {
    const auto cyc_time = static_cast<uint32_t*>(context->userdata)[0];
    ec_dcsync0(slave, true, cyc_time, 0U);
    return 0;
  };

  if (_sync_mode == SYNC_MODE::DC)
    for (int cnt = 1; cnt <= ec_slavecount; cnt++) ec_slave[cnt].PO2SOconfigx = dc_config;

  ec_configdc();

  _io_map.resize(dev_num);
  ec_config_map(_io_map.get());

  ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE * 4);
  ec_readstate();
  ec_slave[0].state = EC_STATE_OPERATIONAL;
  ec_writestate(0);

  const auto expected_wkc = ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC;
  const auto cycle_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _send_cycle;
  _is_running = true;
  std::queue<driver::TxDatagram>().swap(_send_buf);
  ecat_init();
  _ecat_thread = std::thread([this, expected_wkc, cycle_time] {
    ecat_run(this->_high_precision, &this->_is_open, &this->_is_running, expected_wkc, cycle_time, this->_send_mtx, this->_send_buf, this->_io_map,
             std::move(this->_on_lost));
  });

  std::this_thread::sleep_for(std::chrono::milliseconds(100));

  ec_statecheck(0, EC_STATE_OPERATIONAL, EC_TIMEOUTSTATE * 5);

  if (ec_slave[0].state != EC_STATE_OPERATIONAL) {
    _is_running = false;
    if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
    throw std::runtime_error("One ore more slaves are not responding");
  }

  if (_sync_mode == SYNC_MODE::FREE_RUN)
    for (int cnt = 1; cnt <= ec_slavecount; cnt++) dc_config(&ecx_context, static_cast<uint16_t>(cnt));

  _is_open.store(true);
}

void SOEMLink::close() {
  if (!is_open()) return;

  while (!_send_buf.empty()) std::this_thread::sleep_for(std::chrono::milliseconds(1));

  _is_open.store(false);

  _is_running = false;
  if (this->_ecat_thread.joinable()) this->_ecat_thread.join();

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
  return std::make_unique<SOEMLink>(_high_precision, _ifname, _device_num, _sync0_cycle, _send_cycle, std::move(_callback), _sync_mode);
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
