// File: drive.hpp
// Project: internal
// Created Date: 24/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/phase.hpp"

namespace autd3::internal {
struct Drive final {
  Phase phase;
  EmitIntensity intensity;

  explicit Drive(const Phase phase, const EmitIntensity intensity) : phase(phase), intensity(intensity) {}
  explicit Drive(const Phase phase, const uint8_t intensity) : phase(phase), intensity(intensity) {}

  auto operator<=>(const Drive&) const = default;
};

}  // namespace autd3::internal
