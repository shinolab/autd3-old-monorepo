// File: silencer_config.hpp
// Project: autd3
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3 {

struct SilencerConfig {
  SilencerConfig() noexcept : SilencerConfig(10, 4096) {}
  explicit SilencerConfig(const uint16_t step, const uint16_t cycle) noexcept : step(step), cycle(cycle) {}

  static SilencerConfig none() noexcept { return SilencerConfig(4096, 4096); }

  uint16_t step;
  uint16_t cycle;
};

}  // namespace autd3
