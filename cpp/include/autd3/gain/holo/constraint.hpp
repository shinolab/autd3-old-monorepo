// File: constraint.hpp
// Project: holo
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain::holo {

/**
 * @brief Amplitude constraint
 */
class EmissionConstraint final {
 public:
  /**
   * @brief Do nothing (this is equivalent to `Clamp(0, 1)`)
   */
  static EmissionConstraint dont_care() { return EmissionConstraint{internal::native_methods::AUTDGainHoloConstraintDotCare()}; }

  /**
   * @brief Normalize the value by dividing the maximum value
   */
  static EmissionConstraint normalize() { return EmissionConstraint{internal::native_methods::AUTDGainHoloConstraintNormalize()}; }

  /**
   * @brief Set all amplitudes to the specified value
   * @param value amplitude
   */
  static EmissionConstraint uniform(const internal::EmitIntensity value) {
    return EmissionConstraint{internal::native_methods::AUTDGainHoloConstraintUniform(value.value())};
  }

  /**
   * @brief Set all amplitudes to the specified value
   * @param value amplitude
   */
  static EmissionConstraint uniform(const uint8_t value) { return uniform(internal::EmitIntensity(value)); }

  /**
   * @brief Clamp all amplitudes to the specified range
   *
   * @param min_v minimum amplitude
   * @param max_v maximum amplitude
   */
  static EmissionConstraint clamp(const internal::EmitIntensity min_v, const internal::EmitIntensity max_v) {
    return EmissionConstraint{internal::native_methods::AUTDGainHoloConstraintClamp(min_v.value(), max_v.value())};
  }

  /**
   * @brief Clamp all amplitudes to the specified range
   *
   * @param min_v minimum amplitude
   * @param max_v maximum amplitude
   */
  static EmissionConstraint clamp(const uint8_t min_v, const uint8_t max_v) {
    return clamp(internal::EmitIntensity(min_v), internal::EmitIntensity(max_v));
  }

  [[nodiscard]] internal::native_methods::EmissionConstraintPtr ptr() const { return _ptr; }

 private:
  explicit EmissionConstraint(const internal::native_methods::EmissionConstraintPtr ptr) : _ptr(ptr) {}

  internal::native_methods::EmissionConstraintPtr _ptr;
};

}  // namespace autd3::gain::holo
