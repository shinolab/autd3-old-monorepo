// File: soem.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 27/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <functional>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "autd3/core/link.hpp"
#include "autd3/core/utils/osal_timer/timer_strategy.hpp"
#include "autd3/driver/debug_level.hpp"
#include "autd3/link/builder.hpp"
#include "autd3/link/ecat.hpp"

namespace autd3 {
using core::TimerStrategy;
using link::SyncMode;
}  // namespace autd3

namespace autd3::link {

/**
 * @brief Link using [SOEM](https://github.com/OpenEtherCATSociety/SOEM)
 */
class SOEM : public LinkBuilder<SOEM> {
 public:
  /**
   * @brief Enumerate Ethernet adapters of the computer.
   */
  static std::vector<EtherCATAdapter> enumerate_adapters();

  /**
   * @brief Constructor
   */
  SOEM()
      : LinkBuilder(core::Milliseconds(0)),
        _timer_strategy(TimerStrategy::Sleep),
        _sync0_cycle(2),
        _send_cycle(2),
        _callback(nullptr),
        _sync_mode(SyncMode::FreeRun),
        _state_check_interval(std::chrono::milliseconds(100)) {}

  /**
   * @brief Set network interface name. (e.g. eth0)
   * @details If ifname is empty (default), the device to which AUTD is connected will be  automatically selected. Available Network interface names
   * are obtained by enumerate_adapters().
   */
  SOEM& ifname(std::string ifname) {
    _ifname = std::move(ifname);
    return *this;
  }

  /**
   * @brief Set send buffer size (unlimited if 0).
   */
  SOEM& buf_size(const size_t size) {
    _buf_size = size;
    return *this;
  }

  /**
   * @brief Set callback function which is called when the link is lost
   */
  SOEM& on_lost(std::function<void(std::string)> callback) {
    _callback = std::move(callback);
    return *this;
  }

  /**
   * @brief Set EtherCAT Sync0 cycle in units of 500us
   */
  SOEM& sync0_cycle(const uint16_t cycle) {
    _sync0_cycle = cycle;
    return *this;
  }

  /**
   * @brief Set EtherCAT send cycle in units of 500us
   */
  SOEM& send_cycle(const uint16_t cycle) {
    _send_cycle = cycle;
    return *this;
  }

  SOEM& timer_strategy(const TimerStrategy timer_strategy) {
    _timer_strategy = timer_strategy;
    return *this;
  }

  /**
   * @brief Set EtherCAT sync mode.
   */
  SOEM& sync_mode(const SyncMode sync_mode) {
    _sync_mode = sync_mode;
    return *this;
  }

  /**
   * @brief Set Debug level (for debug)
   */
  SOEM& debug_level(const driver::DebugLevel level) {
    _debug_level = level;
    return *this;
  }

  /**
   * @brief Set Debug log func (for debug)
   * @details The log will be written to stdout by default
   */
  SOEM& debug_log_func(std::function<void(std::string)> out, std::function<void()> flush) {
    _debug_out = std::move(out);
    _debug_flush = std::move(flush);
    return *this;
  }

  /**
   * @brief Set EtherCAT state check interval.
   */
  template <typename Rep, typename Period>
  SOEM& state_check_interval(const std::chrono::duration<Rep, Period> interval) {
    _state_check_interval = std::chrono::duration_cast<std::chrono::milliseconds>(interval);
    return *this;
  }

  ~SOEM() = default;
  SOEM(const SOEM& v) noexcept = default;
  SOEM& operator=(const SOEM& obj) = default;
  SOEM(SOEM&& obj) = default;
  SOEM& operator=(SOEM&& obj) = default;

 protected:
  core::LinkPtr build_() override;

 private:
  TimerStrategy _timer_strategy;
  std::string _ifname;
  size_t _buf_size{0};
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;
  std::function<void(std::string)> _callback;
  SyncMode _sync_mode;
  std::chrono::milliseconds _state_check_interval;
  driver::DebugLevel _debug_level{driver::DebugLevel::Info};
  std::function<void(std::string)> _debug_out{nullptr};
  std::function<void()> _debug_flush{nullptr};
};
}  // namespace autd3::link
