// File: synchronize.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/core/interface.hpp"
#include "autd3/driver/driver.hpp"

namespace autd3::core {

/**
 * @brief Synchronize
 */
struct Synchronize final : DatagramBody {
  Synchronize() noexcept = default;

  void init() override {}

  void pack(const std::unique_ptr<const driver::Driver>& driver, const Geometry& geometry, driver::TxDatagram& tx) override {
    std::vector<uint16_t> cycles;
    std::for_each(geometry.begin(), geometry.end(), [&](const auto& dev) {
      std::transform(dev.begin(), dev.end(), std::back_inserter(cycles), [](const core::Transducer& tr) { return tr.cycle(); });
    });
    driver->sync(cycles.data(), tx);
  }

  bool is_finished() const override { return true; }
};

}  // namespace autd3::core
