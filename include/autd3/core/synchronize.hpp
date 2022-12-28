// File: synchronize.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/driver.hpp"

namespace autd3::core {

/**
 * @brief DatagramBody for synchronization
 */
struct Synchronize final : DatagramBody {
  Synchronize() noexcept = default;

  bool init() override { return true; }

  bool pack(const std::unique_ptr<const driver::Driver>& driver, const std::unique_ptr<const Mode>& mode, const Geometry& geometry,
            driver::TxDatagram& tx) override {
    std::vector<uint16_t> cycles;
    cycles.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(cycles), [](const Transducer& tr) { return tr.cycle(); });
    return mode->pack_sync(driver, cycles, tx);
  }

  [[nodiscard]] bool is_finished() const override { return true; }
};

}  // namespace autd3::core
