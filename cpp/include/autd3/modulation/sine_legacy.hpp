// File: sine_legacy.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 21/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>

#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/transform.hpp"

namespace autd3::modulation {

/**
 * @brief Sine wave modulation
 */
class SineLegacy final : public internal::Modulation {
 public:
  /**
   * @brief Constructor.
   * @details The sine wave is defined as `amp / 2 * sin(2π * freq * t) + offset`, where `t` is time, and `amp = 1`, `offset
   * = 0.5` by default.
   *
   * @param freq Frequency of sine wave
   */
  explicit SineLegacy(const double freq) : _freq(freq) {}

  AUTD3_IMPL_WITH_CACHE_MODULATION
  AUTD3_IMPL_WITH_RADIATION_PRESSURE(SineLegacy)
  AUTD3_IMPL_WITH_TRANSFORM_MODULATION

  AUTD3_DEF_PARAM(SineLegacy, double, amp)
  AUTD3_DEF_PARAM(SineLegacy, double, offset)

  AUTD3_IMPL_MOD_PROP(SineLegacy)

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = internal::native_methods::AUTDModulationSineLegacy(_freq);
    if (_amp.has_value()) ptr = AUTDModulationSineLegacyWithAmp(ptr, _amp.value());
    if (_offset.has_value()) ptr = AUTDModulationSineLegacyWithOffset(ptr, _offset.value());
    if (_freq_div.has_value()) ptr = AUTDModulationSineLegacyWithSamplingFrequencyDivision(ptr, _freq_div.value());
    return ptr;
  }

 private:
  double _freq;
  std::optional<double> _amp;
  std::optional<double> _offset;
  std::optional<uint32_t> _freq_div;
};
}  // namespace autd3::modulation
