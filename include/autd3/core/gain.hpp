// File: gain.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <vector>

#include "autd3/core/datagram.hpp"
#include "autd3/core/utils/iter.hpp"
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
    switch (geometry.mode()) {
      case Mode::Legacy:
        return std::make_unique<driver::Gain<driver::Legacy>>(calc(geometry));
      case Mode::Advanced:
        return std::make_unique<driver::Gain<driver::Advanced>>(calc(geometry), geometry.cycles());
      case Mode::AdvancedPhase:
        return std::make_unique<driver::Gain<driver::AdvancedPhase>>(calc(geometry), geometry.cycles());
    }
    throw std::runtime_error("Unreachable!");
  }

  template <class Fn>
  static std::vector<driver::Drive> transform(const Geometry& geometry, Fn func) {
    std::vector<driver::Drive> drives;
    drives.resize(geometry.num_transducers());
    core::transform(geometry.begin(), geometry.end(), drives.begin(), func);
    return drives;
  }
};

}  // namespace autd3::core
