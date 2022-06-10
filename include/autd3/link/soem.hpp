// File: soem.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/06/2022
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
   * @param ifname Network interface name. (e.g. eth0)
   * @param device_num The number of AUTD you connected.
   * @details Available Network interface names are obtained by enumerate_adapters(). The numbers of connected devices can be obtained by
   * Geometry::num_devices().
   */
  SOEM(std::string ifname, const size_t device_num)
      : _high_precision(false), _ifname(std::move(ifname)), _device_num(device_num), _cycle_ticks(1), _callback(nullptr) {}

  /**
   * @brief Set callback function which is called when the link is lost
   */
  SOEM& on_lost(std::function<void(std::string)> callback) {
    _callback = std::move(callback);
    return *this;
  }

  /**
   * @brief Set EtherCAT Sync0 cycle ticks
   */
  SOEM& cycle_ticks(const uint16_t cycle_ticks) {
    _cycle_ticks = cycle_ticks;
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

  ~SOEM() = default;
  SOEM(const SOEM& v) noexcept = delete;
  SOEM& operator=(const SOEM& obj) = delete;
  SOEM(SOEM&& obj) = delete;
  SOEM& operator=(SOEM&& obj) = delete;

 private:
  bool _high_precision;
  std::string _ifname;
  size_t _device_num;
  uint16_t _cycle_ticks;
  std::function<void(std::string)> _callback;
};
}  // namespace autd3::link
