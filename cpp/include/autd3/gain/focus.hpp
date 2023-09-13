// File: focus.hpp
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
#include "autd3/internal/utils.hpp"

namespace autd3::gain {

/**
 * @brief Gain to produce single focal point
 */
class Focus final : public internal::Gain {
 public:
  explicit Focus(internal::Vector3 p) : _p(std::move(p)) {}

  AUTD3_IMPL_WITH_CACHE_GAIN(Focus)
  AUTD3_IMPL_WITH_TRANSFORM_GAIN(Focus)

  AUTD3_DEF_PARAM(Focus, double, amp)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainFocus(_p.x(), _p.y(), _p.z());
    if (_amp.has_value()) ptr = AUTDGainFocusWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  internal::Vector3 _p;
  std::optional<double> _amp;
};

}  // namespace autd3::gain
