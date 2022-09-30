// File: simulator.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/core/link.hpp"

namespace autd3::link {

/**
 * \brief link for autd3::extra::simulator::Simulator
 */
class Simulator {
 public:
  /**
   * @brief Constructor
   */
  Simulator() : _port(50632) {}
  ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = delete;
  Simulator& operator=(Simulator&& obj) = delete;

  Simulator& port(const uint16_t port) {
    _port = port;
    return *this;
  }
  core::LinkPtr build();

 private:
  uint16_t _port;
};

}  // namespace autd3::link
