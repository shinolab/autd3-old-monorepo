// File: emulator.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/core/geometry/geometry.hpp"
#include "autd3/core/link.hpp"

namespace autd3::link {

/**
 * \brief link for [autd-emulator](https://github.com/shinolab/autd-emulator)
 */
template <typename T = core::LegacyTransducer>
class Emulator {
 public:
  /**
   * @brief Constructor
   * @param geometry geometry
   */
  explicit Emulator(const core::Geometry<T>& geometry) : _port(50632), _geometry(geometry) {}
  ~Emulator() = default;
  Emulator(const Emulator& v) noexcept = delete;
  Emulator& operator=(const Emulator& obj) = delete;
  Emulator(Emulator&& obj) = delete;
  Emulator& operator=(Emulator&& obj) = delete;

  Emulator& port(const uint16_t port) {
    _port = port;
    return *this;
  }
  core::LinkPtr build();

 private:
  uint16_t _port;
  const core::Geometry<T>& _geometry;
};

}  // namespace autd3::link
