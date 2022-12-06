// File: soem.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/12/2022
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

namespace autd3::link {

/**
 * \brief EtherCAT adapter information for SOEM
 */
struct EtherCATAdapter final {
  EtherCATAdapter(std::string desc, std::string name) : desc(std::move(desc)), name(std::move(name)) {}

  std::string desc;
  std::string name;
};

enum class SYNC_MODE { FREE_RUN, DC };

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
      : _high_precision(false),
        _sync0_cycle(2),
        _send_cycle(2),
        _callback(nullptr),
        _sync_mode(SYNC_MODE::DC),
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
   * @brief Set high precision mode.
   * @details The high precision mode provides more precise timer control but may increase CPU load. Only Windows is affected by this setting.
   */
  SOEM& high_precision(const bool high_precision) {
    _high_precision = high_precision;
    return *this;
  }

  /**
   * @brief Set EtherCAT sync mode.
   */
  SOEM& sync_mode(const SYNC_MODE sync_mode) {
    _sync_mode = sync_mode;
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
  bool _high_precision;
  std::string _ifname;
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;
  std::function<void(std::string)> _callback;
  SYNC_MODE _sync_mode;
  std::chrono::milliseconds _state_check_interval;
};
}  // namespace autd3::link
