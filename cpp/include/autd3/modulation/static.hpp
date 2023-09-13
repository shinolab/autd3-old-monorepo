// File: static.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once


#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/transform.hpp"

namespace autd3::modulation {

/**
 * @brief Without modulation
 */
class Static final : public internal::Modulation {
 public:
  Static() = default;

  AUTD3_IMPL_WITH_CACHE_MODULATION
  AUTD3_IMPL_WITH_RADIATION_PRESSURE
  AUTD3_IMPL_WITH_TRANSFORM_MODULATION

  AUTD3_DEF_PARAM(Static, double, amp)

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = internal::native_methods::AUTDModulationStatic();
    if (_amp.has_value()) ptr = AUTDModulationStaticWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  std::optional<double> _amp;
};
}  // namespace autd3::modulation
