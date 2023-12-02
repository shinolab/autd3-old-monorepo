// File: square.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/transform.hpp"

namespace autd3::modulation {

/**
 * @brief Square wave modulation
 */
class Square final : public internal::ModulationWithSamplingConfig<Square>,
                     public IntoCache<Square>,
                     public IntoTransform<Square>,
                     public IntoRadiationPressure<Square> {
 public:
  /**
   * @brief Constructor
   *
   * @param freq Frequency of square wave
   */
  explicit Square(const double freq) : _freq(freq) {}

  AUTD3_DEF_PARAM_INTENSITY(Square, low)
  AUTD3_DEF_PARAM_INTENSITY(Square, high)
  AUTD3_DEF_PARAM(Square, double, duty)

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = internal::native_methods::AUTDModulationSquare(_freq);
    if (_low.has_value()) ptr = AUTDModulationSquareWithLow(ptr, _low.value().value());
    if (_high.has_value()) ptr = AUTDModulationSquareWithHigh(ptr, _high.value().value());
    if (_duty.has_value()) ptr = AUTDModulationSquareWithDuty(ptr, _duty.value());
    if (_config.has_value())
      ptr = AUTDModulationSquareWithSamplingConfig(ptr, static_cast<internal::native_methods::SamplingConfiguration>(_config.value()));
    return ptr;
  }

 private:
  double _freq;
  std::optional<internal::EmitIntensity> _low;
  std::optional<internal::EmitIntensity> _high;
  std::optional<double> _duty;
};

}  // namespace autd3::modulation
