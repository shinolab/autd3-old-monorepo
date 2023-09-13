// File: uniform.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <optional>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::gain {

/**
 * @brief Gain to set amp and phase uniformly
 */
class Uniform final : public internal::Gain {
 public:
  explicit Uniform(const double amp) : _amp(amp) {}

  AUTD3_IMPL_WITH_CACHE_GAIN(Uniform)
  AUTD3_IMPL_WITH_TRANSFORM_GAIN(Uniform)

  AUTD3_DEF_PARAM(Uniform, double, phase)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainUniform(_amp);
    if (_phase.has_value()) ptr = AUTDGainUniformWithPhase(ptr, _phase.value());
    return ptr;
  }

 private:
  double _amp;
  std::optional<double> _phase;
};
}  // namespace autd3::gain
