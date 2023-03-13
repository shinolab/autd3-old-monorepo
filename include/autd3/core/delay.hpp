// File: delay.hpp
// Project: core
// Created Date: 01/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <memory>
#include <utility>
#include <vector>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/mod_delay.hpp"

namespace autd3::core {

/**
 * @brief ModDelayConfig is a SpecialData to configure modulation delay
 */

struct ModDelayConfig final : SpecialData {
  ModDelayConfig() noexcept : SpecialData(std::chrono::nanoseconds::zero(), std::make_unique<NullHeader>(), std::make_unique<ModDelayConfigBody>()) {}

 private:
  struct ModDelayConfigBody final : DatagramBody {
    ModDelayConfigBody() = default;
    ~ModDelayConfigBody() override = default;

    std::unique_ptr<driver::Operation> operation(const Geometry& geometry) override {
      std::vector<uint16_t> delays;
      delays.reserve(geometry.num_transducers());
      std::transform(geometry.begin(), geometry.end(), std::back_inserter(delays), [](const Transducer& tr) { return tr.mod_delay; });
      return std::make_unique<driver::ModDelay>(std::move(delays));
    }
  };
};

}  // namespace autd3::core
