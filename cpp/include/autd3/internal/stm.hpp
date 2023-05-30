// File: stm.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/def.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class FocusSTM final : public Body {
 public:
  struct Focus {
    Vector3 point;
    uint8_t shift;

    explicit Focus(Vector3 point, const uint8_t shift = 0) : point(std::move(point)), shift(shift) {}
    ~Focus() = default;
    Focus(const Focus& v) noexcept = default;
    Focus& operator=(const Focus& obj) = default;
    Focus(Focus&& obj) = default;
    Focus& operator=(Focus&& obj) = default;
  };

  using value_type = Focus;

  FocusSTM() : Body(native_methods::AUTDFocusSTM()) {}
  ~FocusSTM() override {
    if (_ptr != nullptr) native_methods::AUTDDeleteFocusSTM(_ptr);
  }

  void add(const Vector3& point, const uint8_t duty_shift = 0) { native_methods::AUTDFocusSTMAdd(_ptr, point.x(), point.y(), point.z(), duty_shift); }

  void push_back(const value_type& v) { add(v.point, v.shift); }

  [[nodiscard]] double frequency() const { return native_methods::AUTDFocusSTMFrequency(_ptr); }
  double set_frequency(const double freq) const { return native_methods::AUTDFocusSTMSetFrequency(_ptr, freq); }

  [[nodiscard]] double sampling_frequency() const { return native_methods::AUTDFocusSTMSamplingFrequency(_ptr); }

  [[nodiscard]] uint32_t sampling_frequency_division() const { return native_methods::AUTDFocusSTMSamplingFrequencyDivision(_ptr); }

  void set_sampling_frequency(const uint32_t value) const { native_methods::AUTDFocusSTMSetSamplingFrequencyDivision(_ptr, value); }

  [[nodiscard]] std::optional<uint16_t> start_idx() const {
    const auto idx = native_methods::AUTDFocusSTMGetStartIdx(_ptr);
    return idx < 0 ? std::nullopt : std::optional(static_cast<uint16_t>(idx));
  }
  void set_start_idx(const std::optional<uint16_t> start_idx) const {
    const int32_t idx = start_idx.has_value() ? static_cast<int32_t>(start_idx.value()) : -1;
    native_methods::AUTDFocusSTMSetStartIdx(_ptr, idx);
  }

  [[nodiscard]] std::optional<uint16_t> finish_idx() const {
    const auto idx = native_methods::AUTDFocusSTMGetFinishIdx(_ptr);
    return idx < 0 ? std::nullopt : std::optional(static_cast<uint16_t>(idx));
  }
  void set_finish_idx(const std::optional<uint16_t> finish_idx) const {
    const int32_t idx = finish_idx.has_value() ? static_cast<int32_t>(finish_idx.value()) : -1;
    native_methods::AUTDFocusSTMSetFinishIdx(_ptr, idx);
  }
};

class GainSTM final : public Body {
 public:
  GainSTM() : Body(native_methods::AUTDGainSTM()) {}
  ~GainSTM() override {
    if (_ptr != nullptr) native_methods::AUTDDeleteGainSTM(_ptr);
  }

  template <typename G>
  void add(G&& gain) {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    native_methods::AUTDGainSTMAdd(_ptr, gain.ptr());
    gain.set_released();
  }

  [[nodiscard]] double frequency() const { return native_methods::AUTDGainSTMFrequency(_ptr); }
  double set_frequency(const double freq) const { return native_methods::AUTDGainSTMSetFrequency(_ptr, freq); }

  [[nodiscard]] double sampling_frequency() const { return native_methods::AUTDGainSTMSamplingFrequency(_ptr); }

  [[nodiscard]] uint32_t sampling_frequency_division() const { return native_methods::AUTDGainSTMSamplingFrequencyDivision(_ptr); }

  void set_sampling_frequency(const uint32_t value) const { native_methods::AUTDGainSTMSetSamplingFrequencyDivision(_ptr, value); }

  [[nodiscard]] std::optional<uint16_t> start_idx() const {
    const auto idx = native_methods::AUTDGainSTMGetStartIdx(_ptr);
    return idx < 0 ? std::nullopt : std::optional(static_cast<uint16_t>(idx));
  }
  void set_start_idx(const std::optional<uint16_t> start_idx) const {
    const int32_t idx = start_idx.has_value() ? static_cast<int32_t>(start_idx.value()) : -1;
    native_methods::AUTDGainSTMSetStartIdx(_ptr, idx);
  }

  [[nodiscard]] std::optional<uint16_t> finish_idx() const {
    const auto idx = native_methods::AUTDGainSTMGetFinishIdx(_ptr);
    return idx < 0 ? std::nullopt : std::optional(static_cast<uint16_t>(idx));
  }
  void set_finish_idx(const std::optional<uint16_t> finish_idx) const {
    const int32_t idx = finish_idx.has_value() ? static_cast<int32_t>(finish_idx.value()) : -1;
    native_methods::AUTDGainSTMSetFinishIdx(_ptr, idx);
  }
};

}  // namespace autd3::internal
