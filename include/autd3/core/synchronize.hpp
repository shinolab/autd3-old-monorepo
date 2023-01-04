// File: synchronize.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/datagram.hpp"
#include "autd3/driver/driver.hpp"

namespace autd3::core {

/**
 * @brief DatagramBody for synchronization
 */
struct Synchronize final : DatagramBody {
  Synchronize() noexcept = default;

  bool init() override { return true; }

  bool pack(const Mode mode, const Geometry& geometry, driver::TxDatagram& tx) override {
    if (mode == Mode::Legacy) return driver::Sync<driver::Legacy>().cycles(geometry.cycles()).pack(tx);
    return driver::Sync<driver::Normal>().cycles(geometry.cycles()).pack(tx);
  }

  [[nodiscard]] bool is_finished() const override { return true; }
};

}  // namespace autd3::core
