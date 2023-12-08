// File: sine.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/12/2023
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

class Fourier;

/**
 * @brief Sine wave modulation
 */
class Sine final : public internal::ModulationWithSamplingConfig<Sine>,
                   public IntoCache<Sine>,
                   public IntoTransform<Sine>,
                   public IntoRadiationPressure<Sine> {
 public:
  /**
   * @brief Constructor.
   * @details The sine wave is defined as `amp / 2 * sin(2π * freq * t + phase)
   * + offset`, where `t` is time, and `amp = 1`, `offset = 0.5` by default.
   *
   * @param freq Frequency of sine wave
   */
  explicit Sine(const double freq) : _freq(freq) {}

  AUTD3_DEF_PARAM_INTENSITY(Sine, intensity)
  AUTD3_DEF_PARAM_INTENSITY(Sine, offset)
  AUTD3_DEF_PARAM(Sine, double, phase)
  AUTD3_DEF_PARAM(Sine, internal::native_methods::SamplingMode, mode)

  friend Fourier operator+(Sine&& lhs, const Sine& rhs);

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = internal::native_methods::AUTDModulationSine(_freq);
    if (_intensity.has_value()) ptr = AUTDModulationSineWithIntensity(ptr, _intensity.value().value());
    if (_phase.has_value()) ptr = AUTDModulationSineWithPhase(ptr, _phase.value());
    if (_offset.has_value()) ptr = AUTDModulationSineWithOffset(ptr, _offset.value().value());
    if (_config.has_value())
      ptr = AUTDModulationSineWithSamplingConfig(ptr, static_cast<internal::native_methods::SamplingConfiguration>(_config.value()));
    if (_mode.has_value()) ptr = AUTDModulationSineWithMode(ptr, _mode.value());
    return ptr;
  }

 private:
  double _freq;
  std::optional<internal::EmitIntensity> _intensity;
  std::optional<double> _phase;
  std::optional<internal::EmitIntensity> _offset;
  std::optional<internal::native_methods::SamplingMode> _mode;
};

}  // namespace autd3::modulation
