// File: synchronize.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/sync.hpp"

namespace autd3::core {

/**
 * @brief DatagramBody for synchronization
 */
struct Synchronize final : DatagramBody {
  Synchronize() noexcept = default;

  std::unique_ptr<driver::Operation> operation(const Geometry& geometry) override {
    switch (geometry.mode) {
      case Mode::Legacy:
        if (const auto cycles = geometry.cycles(); std::any_of(cycles.begin(), cycles.end(), [](const auto& cycle) { return cycle != 4096; }))
          throw std::runtime_error("Frequency cannot be changed in Legacy mode.");
        return std::make_unique<driver::Sync<driver::Legacy>>();
      case Mode::Advanced:
        return std::make_unique<driver::Sync<driver::Advanced>>(geometry.cycles());
      case Mode::AdvancedPhase:
        return std::make_unique<driver::Sync<driver::AdvancedPhase>>(geometry.cycles());
    }
    throw std::runtime_error("Unreachable!");
  }
};

}  // namespace autd3::core
