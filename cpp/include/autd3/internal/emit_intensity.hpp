// File: emit_intensity.hpp
// Project: internal
// Created Date: 12/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 12/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::internal {
class EmitIntensity {
 public:
  [[nodiscard]] static EmitIntensity new_normalized(const double value) {
    return EmitIntensity(validate<uint16_t>(native_methods::AUTDEmitIntensityNormalizedInto(value)));
  }

  [[nodiscard]] static EmitIntensity new_normalized_corrected_with_alpha(const double value, const double alpha) {
    return EmitIntensity(validate<uint16_t>(native_methods::AUTDEmitIntensityNormalizedCorrectedInto(value, alpha)));
  }

  [[nodiscard]] static EmitIntensity new_normalized_corrected(const double value) {
    return new_normalized_corrected_with_alpha(value, native_methods::DEFAULT_CORRECTED_ALPHA);
  }

  [[nodiscard]] static EmitIntensity new_duty_ratio(const double value) {
    return EmitIntensity(validate<uint16_t>(native_methods::AUTDEmitIntensityDutyRatioInto(value)));
  }

  [[nodiscard]] static EmitIntensity new_pulse_width(const uint16_t value) {
    return EmitIntensity(validate<uint16_t>(native_methods::AUTDEmitIntensityPulseWidthInto(value)));
  }

  [[nodiscard]] double normalized() const noexcept { return native_methods::AUTDEmitIntensityNormalizedFrom(_pulse_width); }

  [[nodiscard]] double duty_ratio() const noexcept { return native_methods::AUTDEmitIntensityDutyRatioFrom(_pulse_width); }

  [[nodiscard]] uint16_t pulse_width() const noexcept { return _pulse_width; }

 private:
  explicit EmitIntensity(const uint16_t pulse_width) : _pulse_width(pulse_width) {}

  uint16_t _pulse_width;
};

}  // namespace autd3::internal
