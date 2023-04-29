// File: autdsoem.cpp
// Project: autdsoem
// Created Date: 23/08/2019
// Author: Shun Suzuki
// -----
// Last Modified: 29/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2019-2020 Shun Suzuki. All rights reserved.
//

#include "link_soem_impl.hpp"

#include <vector>

namespace autd3::link {

bool SOEMLink::is_open() { return _handler->is_open(); }

bool SOEMLink::send(const driver::TxDatagram& tx) { return _handler->send(tx); }

bool SOEMLink::receive(driver::RxDatagram& rx) { return _handler->receive(rx); }

bool SOEMLink::open(const core::Geometry& geometry) {
  if (const auto dev_num = _handler->open(geometry.device_map()); geometry.num_devices() == dev_num) return _handler->is_open();
  _handler->close();
  return false;
}

bool SOEMLink::close() { return _handler->close(); }

core::LinkPtr SOEM::build_() {
  const auto name = "AUTD3";
  if (spdlog::thread_pool() == nullptr) spdlog::init_thread_pool(8192, 1);
  spdlog::sink_ptr sink =
      (_out == nullptr || _flush == nullptr) ? get_default_sink() : std::make_shared<CustomSink<std::mutex>>(std::move(_out), std::move(_flush));
  auto logger = std::make_shared<spdlog::async_logger>(name, std::move(sink), spdlog::thread_pool());
  logger->set_level(static_cast<spdlog::level::level_enum>(_level));
  register_logger(logger);
  return std::make_unique<SOEMLink>(_timeout, _timer_strategy, std::move(_ifname), _buf_size, _sync0_cycle, _send_cycle, std::move(_callback),
                                    _sync_mode, _state_check_interval, logger);
}

std::vector<EtherCATAdapter> SOEM::enumerate_adapters() { return SOEMHandler::enumerate_adapters(); }
}  // namespace autd3::link
