// File: uniform.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <optional>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::gain {

/**
 * @brief Gain to set intensity and phase uniformly
 */
class Uniform final : public internal::Gain, public IntoCache<Uniform>, public IntoTransform<Uniform> {
 public:
  explicit Uniform(const uint8_t intensity) : _intensity(internal::EmitIntensity(intensity)) {}
  explicit Uniform(const internal::EmitIntensity intensity) : _intensity(intensity) {}

  AUTD3_DEF_PARAM(Uniform, internal::Phase, phase)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::geometry::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainUniform(_intensity.value());
    if (_phase.has_value()) ptr = AUTDGainUniformWithPhase(ptr, _phase.value().value());
    return ptr;
  }

 private:
  internal::EmitIntensity _intensity;
  std::optional<internal::Phase> _phase;
};
}  // namespace autd3::gain
