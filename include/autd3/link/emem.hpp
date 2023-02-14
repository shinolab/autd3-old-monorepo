// File: emem.hpp
// Project: link
// Created Date: 04/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/link.hpp"
#include "autd3/link/ecat.hpp"

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
      : _high_precision(false),
        _sync0_cycle(2),
        _send_cycle(2),
        _callback(nullptr),
        _sync_mode(SyncMode::DC),
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

  /**
   * @brief Set high precision mode.
   * @details The high precision mode provides more precise timer control but may increase CPU load. Only Windows is affected by this setting.
   */
  Emem& high_precision(const bool high_precision) {
    _high_precision = high_precision;
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
  bool _high_precision;
  std::string _ifname;
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;
  std::function<void(std::string)> _callback;
  SyncMode _sync_mode;
  std::chrono::milliseconds _state_check_interval;
};

}  // namespace autd3::link
