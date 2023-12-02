// File: focus.hpp
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
#include "autd3/internal/def.hpp"
#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::gain {

/**
 * @brief Gain to produce single focal point
 */
class Focus final : public internal::Gain, public IntoCache<Focus>, public IntoTransform<Focus> {
 public:
  explicit Focus(internal::Vector3 p) : _p(std::move(p)) {}

  AUTD3_DEF_PARAM_INTENSITY(Focus, intensity)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::geometry::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainFocus(_p.x(), _p.y(), _p.z());
    if (_intensity.has_value()) ptr = AUTDGainFocusWithIntensity(ptr, _intensity.value().value());
    return ptr;
  }

 private:
  internal::Vector3 _p;
  std::optional<internal::EmitIntensity> _intensity;
};

}  // namespace autd3::gain
