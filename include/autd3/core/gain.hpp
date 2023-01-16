// File: gain.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/gain.hpp"

namespace autd3::core {

/**
 * @brief Gain controls the duty ratio and phase of each transducer in AUTD devices
 */
struct Gain : DatagramBody {
  Gain() = default;
  ~Gain() override = default;
  Gain(const Gain& v) = default;
  Gain& operator=(const Gain& obj) = default;
  Gain(Gain&& obj) = default;
  Gain& operator=(Gain&& obj) = default;

  /**
   * \brief Calculate duty ratio and phase of each transducer
   * \param geometry Geometry
   */
  virtual std::vector<driver::Drive> calc(const Geometry& geometry) = 0;

  std::unique_ptr<driver::Operation> operation(const Geometry& geometry) override {
    switch (geometry.mode) {
      case Mode::Legacy:
        return std::make_unique<driver::Gain<driver::Legacy>>(calc(geometry));
      case Mode::Normal:
        return std::make_unique<driver::Gain<driver::Normal>>(calc(geometry), geometry.cycles());
      case Mode::NormalPhase:
        return std::make_unique<driver::Gain<driver::NormalPhase>>(calc(geometry), geometry.cycles());
    }
    throw std::runtime_error("Unreachable!");
  }
};

}  // namespace autd3::core
