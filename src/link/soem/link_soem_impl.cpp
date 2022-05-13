// File: autdsoem.cpp
// Project: autdsoem
// Created Date: 23/08/2019
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2019-2020 Hapis Lab. All rights reserved.
//

#include "link_soem_impl.hpp"

#include <cstdint>
#include <memory>
#include <queue>
#include <sstream>
#include <string>
#include <vector>

#include "./ethercat.h"
#include "autd3/link/soem.hpp"
#include "ecat_thread/ecat_thread.hpp"

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

void SOEMLink::open() {
  _user_data = std::make_unique<uint32_t[]>(1);
  _user_data[0] = driver::EC_SYNC0_CYCLE_TIME_NANO_SEC * _cycle_ticks;

  std::queue<driver::TxDatagram>().swap(_send_buf);

  _io_map.resize(_dev_num);

  if (ec_init(_ifname.c_str()) <= 0) {
    std::stringstream ss;
    ss << "No socket connection on " << _ifname;
    throw std::runtime_error(ss.str());
  }

  const auto wc = ec_config_init(0);
  if (wc <= 0) throw std::runtime_error("No slaves found!");

  if (static_cast<size_t>(wc) != _dev_num) {
    std::stringstream ss;
    ss << "The number of slaves you added: " << _dev_num << ", but found: " << wc;
    throw std::runtime_error(ss.str());
  }

  ecx_context.userdata = _user_data.get();
  auto dc_config = [](ecx_contextt* const context, const uint16_t slave) -> int {
    const auto cyc_time = static_cast<uint32_t*>(context->userdata)[0];
    ec_dcsync0(slave, true, cyc_time, 0U);
    return 0;
  };
  for (int cnt = 1; cnt <= ec_slavecount; cnt++) ec_slave[cnt].PO2SOconfigx = dc_config;

  ec_configdc();

  ec_config_map(_io_map.get());

  ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE * 4);
  ec_readstate();
  ec_slave[0].state = EC_STATE_OPERATIONAL;
  ec_writestate(0);

  const auto expected_wkc = ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC;
  const auto cycle_time = driver::EC_SYNC0_CYCLE_TIME_NANO_SEC * _cycle_ticks;
  _is_running = true;
  ecat_init();
  _ecat_thread = std::thread([this, expected_wkc, cycle_time] {
    ecat_run(this->_high_precision, &this->_is_open, &this->_is_running, expected_wkc, cycle_time, this->_send_mtx, this->_send_buf, this->_io_map,
             std::move(this->_on_lost));
  });

  std::this_thread::sleep_for(std::chrono::milliseconds(100));

  ec_statecheck(0, EC_STATE_OPERATIONAL, EC_TIMEOUTSTATE * 5);

  if (ec_slave[0].state != EC_STATE_OPERATIONAL) throw std::runtime_error("One ore more slaves are not responding");

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

core::LinkPtr SOEM::build() { return std::make_unique<SOEMLink>(_high_precision, _ifname, _device_num, _cycle_ticks, std::move(_callback)); }

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
