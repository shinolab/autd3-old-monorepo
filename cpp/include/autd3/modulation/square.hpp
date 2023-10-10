// File: square.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
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
 * @brief Square wave modulation
 */
class Square final : public internal::Modulation, public IntoCache<Square>, public IntoTransform<Square>, public IntoRadiationPressure<Square> {
 public:
  /**
   * @brief Constructor
   *
   * @param freq Frequency of square wave
   */
  explicit Square(const int32_t freq) : _freq(freq) {}

  AUTD3_DEF_PARAM(Square, double, low)
  AUTD3_DEF_PARAM(Square, double, high)
  AUTD3_DEF_PARAM(Square, double, duty)

  AUTD3_IMPL_MOD_PROP(Square)

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = internal::native_methods::AUTDModulationSquare(_freq);
    if (_low.has_value()) ptr = AUTDModulationSquareWithLow(ptr, _low.value());
    if (_high.has_value()) ptr = AUTDModulationSquareWithHigh(ptr, _high.value());
    if (_duty.has_value()) ptr = AUTDModulationSquareWithDuty(ptr, _duty.value());
    if (_freq_div.has_value()) ptr = AUTDModulationSquareWithSamplingFrequencyDivision(ptr, _freq_div.value());
    return ptr;
  }

 private:
  int32_t _freq;
  std::optional<double> _low;
  std::optional<double> _high;
  std::optional<double> _duty;
  std::optional<uint32_t> _freq_div;
};

}  // namespace autd3::modulation
