// File: soem.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

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
   * @param device_num The number of AUTD you connected.
   * @details The numbers of connected devices can be obtained by Geometry::num_devices().
   */
  explicit SOEM(const size_t device_num)
      : _high_precision(false), _device_num(device_num), _sync0_cycle(1), _send_cycle(1), _callback(nullptr), _sync_mode(SYNC_MODE::DC) {}

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

  ~SOEM() = default;
  SOEM(const SOEM& v) noexcept = delete;
  SOEM& operator=(const SOEM& obj) = delete;
  SOEM(SOEM&& obj) = delete;
  SOEM& operator=(SOEM&& obj) = delete;

 private:
  bool _high_precision;
  std::string _ifname;
  size_t _device_num;
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;
  std::function<void(std::string)> _callback;
  SYNC_MODE _sync_mode;
};
}  // namespace autd3::link
