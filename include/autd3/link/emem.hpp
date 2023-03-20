// File: emem.hpp
// Project: link
// Created Date: 04/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 21/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/link.hpp"
#include "autd3/core/utils/osal_timer/timer_strategy.hpp"
#include "autd3/link/ecat.hpp"

namespace autd3 {
using core::TimerStrategy;
using link::SyncMode;
}  // namespace autd3

namespace autd3::link {

/**
 * @brief Link for Experimental Mini EtherCAT Master
 */
class Emem {
 public:
  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] core::LinkPtr build();

  /**
   * @brief Enumerate Ethernet adapters of the computer.
   */
  static std::vector<EtherCATAdapter> enumerate_adapters();

  /**
   * @brief Constructor
   */
  Emem()
      : _timer_strategy(TimerStrategy::Sleep),
        _sync0_cycle(2),
        _send_cycle(2),
        _callback(nullptr),
        _sync_mode(SyncMode::FreeRun),
        _state_check_interval(std::chrono::milliseconds(100)) {}

  ~Emem() = default;
  Emem(const Emem& v) noexcept = delete;
  Emem& operator=(const Emem& obj) = delete;
  Emem(Emem&& obj) = default;
  Emem& operator=(Emem&& obj) = default;

  /**
   * @brief Set network interface name. (e.g. eth0)
   * @details If ifname is empty (default), the device to which AUTD is connected will be  automatically selected. Available Network interface names
   * are obtained by enumerate_adapters().
   */
  Emem& ifname(std::string ifname) {
    _ifname = std::move(ifname);
    return *this;
  }

  /**
   * @brief Set send buffer size (unlimited if 0).
   */
  Emem& buf_size(const size_t size) {
    _buf_size = size;
    return *this;
  }

  /**
   * @brief Set callback function which is called when the link is lost
   */
  Emem& on_lost(std::function<void(std::string)> callback) {
    _callback = std::move(callback);
    return *this;
  }

  /**
   * @brief Set EtherCAT Sync0 cycle in units of 500us
   */
  Emem& sync0_cycle(const uint16_t cycle) {
    _sync0_cycle = cycle;
    return *this;
  }

  /**
   * @brief Set EtherCAT send cycle in units of 500us
   */
  Emem& send_cycle(const uint16_t cycle) {
    _send_cycle = cycle;
    return *this;
  }

  Emem& timer_strategy(const TimerStrategy timer_strategy) {
    _timer_strategy = timer_strategy;
    return *this;
  }

  /**
   * @brief Set EtherCAT sync mode.
   */
  Emem& sync_mode(const SyncMode sync_mode) {
    _sync_mode = sync_mode;
    return *this;
  }

  /**
   * @brief Set EtherCAT state check interval.
   */
  template <typename Rep, typename Period>
  Emem& state_check_interval(const std::chrono::duration<Rep, Period> interval) {
    _state_check_interval = std::chrono::duration_cast<std::chrono::milliseconds>(interval);
    return *this;
  }

 private:
  TimerStrategy _timer_strategy;
  std::string _ifname;
  size_t _buf_size{0};
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;
  std::function<void(std::string)> _callback;
  SyncMode _sync_mode;
  std::chrono::milliseconds _state_check_interval;
};

}  // namespace autd3::link
