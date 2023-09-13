// File: plane.hpp
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
#include "autd3/internal/def.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

/**
 * @brief Gain to produce a plane wave
 */
class Plane final : public internal::Gain {
 public:
  explicit Plane(internal::Vector3 d) : _d(std::move(d)) {}

  AUTD3_IMPL_WITH_CACHE_GAIN(Plane)
  AUTD3_IMPL_WITH_TRANSFORM_GAIN(Plane)

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (from 0 to 1)
   * @return Plane
   */
  void with_amp(const double amp) & { _amp = amp; }

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (from 0 to 1)
   * @return Plane
   */
  [[nodiscard]] Plane&& with_amp(const double amp) && {
    _amp = amp;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainPlane(_d.x(), _d.y(), _d.z());
    if (_amp.has_value()) ptr = AUTDGainPlaneWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  internal::Vector3 _d;
  std::optional<double> _amp;
};

}  // namespace autd3::gain
