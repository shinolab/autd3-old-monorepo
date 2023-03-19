// File: soem.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 19/03/2023
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
#include "autd3/core/osal_timer/timer_strategy.hpp"
#include "autd3/driver/debug_level.hpp"
#include "autd3/link/ecat.hpp"

namespace autd3::link {

/**
 * @brief Link using [SOEM](https://github.com/OpenEtherCATSociety/SOEM)
 */
class SOEM {
 public:
  /**
   * @brief Create SOEM link.
   */
  core::LinkPtr build();

  /**
   * @brief Enumerate Ethernet adapters of the computer.
   */
  static std::vector<EtherCATAdapter> enumerate_adapters();

  /**
   * @brief Constructor
   */
  SOEM()
      : _timer_strategy(core::TimerStrategy::Sleep),
        _sync0_cycle(2),
        _send_cycle(2),
        _callback(nullptr),
        _sync_mode(SyncMode::DC),
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

  /**
   * @brief This function is deprecated.
   */
#ifdef WIN32
  [[deprecated("Please use timer_strategy(autd3::TimerStrategy::BusyWait) instead.")]]
#else
  [[deprecated("This function is meaningless and should be removed.")]]
#endif
  SOEM&
  high_precision(bool) {
    return *this;
  }

  SOEM& timer_strategy(const core::TimerStrategy timer_strategy) {
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
    _level = level;
    return *this;
  }

  /**
   * @brief Set Debug log func (for debug)
   * @details The log will be written to stdout by default
   */
  SOEM& debug_log_func(std::function<void(std::string)> out, std::function<void()> flush) {
    _out = std::move(out);
    _flush = std::move(flush);
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

 private:
  core::TimerStrategy _timer_strategy;
  std::string _ifname;
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;
  std::function<void(std::string)> _callback;
  SyncMode _sync_mode;
  std::chrono::milliseconds _state_check_interval;
  driver::DebugLevel _level{driver::DebugLevel::Info};
  std::function<void(std::string)> _out{nullptr};
  std::function<void()> _flush{nullptr};
};
}  // namespace autd3::link
