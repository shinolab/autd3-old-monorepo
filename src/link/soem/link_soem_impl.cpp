// File: autdsoem.cpp
// Project: autdsoem
// Created Date: 23/08/2019
// Author: Shun Suzuki
// -----
// Last Modified: 18/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2019-2020 Shun Suzuki. All rights reserved.
//

#include "link_soem_impl.hpp"

namespace autd3::link {

bool SOEMLink::is_open() { return _handler->is_open(); }

bool SOEMLink::send(const driver::TxDatagram& tx) { return _handler->send(tx); }

bool SOEMLink::receive(driver::RxDatagram& rx) { return _handler->receive(rx); }

bool SOEMLink::open(const core::Geometry& geometry) {
  const auto dev_num = _handler->open(1);
  if (geometry.num_devices() == dev_num) return true;
  _handler->close();
  spdlog::error("The number of slaves you configured: {}, but found: {}", dev_num, dev_num);
  return false;
}

bool SOEMLink::close() { return _handler->close(); }

core::LinkPtr SOEM::build() {
  return std::make_unique<SOEMLink>(_high_precision, std::move(_ifname), _sync0_cycle, _send_cycle, std::move(_callback), _sync_mode,
                                    _state_check_interval);
}

std::vector<EtherCATAdapter> SOEM::enumerate_adapters() { return SOEMHandler::enumerate_adapters(); }
}  // namespace autd3::link
