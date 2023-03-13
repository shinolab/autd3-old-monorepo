// File: stop.hpp
// Project: core
// Created Date: 13/03/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/core/amplitudes.hpp"
#include "autd3/core/datagram.hpp"
#include "autd3/core/silencer_config.hpp"

namespace autd3::core {

/**
 * @brief Stop is a SpecialData to stop ultrasound output
 */
struct Stop final : SpecialData {
  Stop() noexcept
      : SpecialData(std::chrono::nanoseconds::zero(), std::make_unique<SilencerConfig>(), std::make_unique<Amplitudes>(driver::autd3_float_t{0})) {}
};

}  // namespace autd3::core
