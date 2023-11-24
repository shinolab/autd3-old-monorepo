// File: sampling_config.hpp
// Project: internal
// Created Date: 24/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {
class STM;
class Modulation;

class SamplingConfiguration final {
 public:
  friend class STM;
  friend class Modulation;

  [[nodiscard]] static SamplingConfiguration new_with_frequency(const double f) {
    return SamplingConfiguration(validate(native_methods::AUTDSamplingConfigNewWithFrequency(f)));
  }

  [[nodiscard]] static SamplingConfiguration new_with_frequency_division(const uint32_t div) {
    return SamplingConfiguration(validate(native_methods::AUTDSamplingConfigNewWithFrequencyDivision(div)));
  }
  template <typename Rep, typename Period>
  [[nodiscard]] static SamplingConfiguration new_with_period(const std::chrono::duration<Rep, Period> period) {
    return SamplingConfiguration(validate(native_methods::AUTDSamplingConfigNewWithPeriod(
        static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(period).count()))));
  }

  [[nodiscard]] double frequency() const { return native_methods::AUTDSamplingConfigFrequency(_internal); }

  [[nodiscard]] uint32_t frequency_division() const { return native_methods::AUTDSamplingConfigFrequencyDivision(_internal); }

  [[nodiscard]] std::chrono::nanoseconds period() const { return std::chrono::nanoseconds(native_methods::AUTDSamplingConfigPeriod(_internal)); }

  explicit operator native_methods::SamplingConfiguration() const { return _internal; }

 private:
  explicit SamplingConfiguration(const native_methods::SamplingConfiguration internal_) : _internal(internal_) {}

  native_methods::SamplingConfiguration _internal;
};

}  // namespace autd3::internal
