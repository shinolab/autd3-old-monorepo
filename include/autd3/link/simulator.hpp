// File: simulator.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <utility>

#include "autd3/core/link.hpp"

namespace autd3::link {

/**
 * \brief link for Simulator
 */
class Simulator {
 public:
  /**
   * @brief Constructor
   */
  explicit Simulator() noexcept : _port(50632), _ip_addr("127.0.0.1") {}
  ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = default;

  Simulator& port(const uint16_t port) {
    _port = port;
    return *this;
  }

  Simulator& ip_addr(std::string ip_addr) {
    _ip_addr = std::move(ip_addr);
    return *this;
  }

  [[nodiscard]] core::LinkPtr build() const;

 private:
  uint16_t _port;
  std::string _ip_addr;
};

}  // namespace autd3::link
