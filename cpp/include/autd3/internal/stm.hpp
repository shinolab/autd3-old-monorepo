// File: stm.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <memory>
#include <ranges>

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/def.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class STM : public Datagram {
 public:
  explicit STM(const std::optional<double> freq, const std::optional<double> sampling_freq, const std::optional<uint32_t> sampling_freq_div,
               const std::optional<std::chrono::nanoseconds> sampling_period)
      : _freq(freq), _sampling_freq(sampling_freq), _sampling_freq_div(sampling_freq_div), _sampling_period(sampling_period) {}

  STM(const STM& obj) = default;
  STM& operator=(const STM& obj) = default;
  STM(STM&& obj) = default;
  STM& operator=(STM&& obj) = default;
  ~STM() override = default;

  [[nodiscard]] std::optional<uint16_t> finish_idx() const {
    const auto idx = AUTDSTMPropsFinishIdx(props());
    return idx < 0 ? std::nullopt : std::optional(static_cast<uint16_t>(idx));
  }

  [[nodiscard]] std::optional<uint16_t> start_idx() const {
    const auto idx = AUTDSTMPropsStartIdx(props());
    return idx < 0 ? std::nullopt : std::optional(static_cast<uint16_t>(idx));
  }

 protected:
  [[nodiscard]] native_methods::STMPropsPtr props() const {
    native_methods::STMPropsPtr ptr{nullptr};
    if (_freq.has_value()) ptr = native_methods::AUTDSTMProps(_freq.value());
    if (_sampling_freq.has_value()) ptr = native_methods::AUTDSTMPropsWithSamplingFreq(_sampling_freq.value());
    if (_sampling_freq_div.has_value()) ptr = native_methods::AUTDSTMPropsWithSamplingFreqDiv(_sampling_freq_div.value());
    if (_sampling_period.has_value()) ptr = native_methods::AUTDSTMPropsWithSamplingPeriod(_sampling_period.value().count());
    if (ptr._0 == nullptr) throw std::runtime_error("unreachable!");
    ptr = AUTDSTMPropsWithStartIdx(ptr, _start_idx);
    ptr = AUTDSTMPropsWithFinishIdx(ptr, _finish_idx);
    return ptr;
  }

  [[nodiscard]] double frequency_from_size(const size_t size) const { return AUTDSTMPropsFrequency(props(), size); }

  [[nodiscard]] double sampling_frequency_from_size(const size_t size) const { return AUTDSTMPropsSamplingFrequency(props(), size); }

  [[nodiscard]] uint32_t sampling_frequency_division_from_size(const size_t size) const {
    return AUTDSTMPropsSamplingFrequencyDivision(props(), size);
  }

  [[nodiscard]] std::chrono::nanoseconds sampling_period_from_size(const size_t size) const {
    return std::chrono::nanoseconds(AUTDSTMPropsSamplingPeriod(props(), size));
  }

  std::optional<double> _freq;
  std::optional<double> _sampling_freq;
  std::optional<uint32_t> _sampling_freq_div;
  std::optional<std::chrono::nanoseconds> _sampling_period;
  int32_t _start_idx{-1};
  int32_t _finish_idx{-1};
};

/**
 * @brief Control point for FocusSTM
 */
struct ControlPoint {
  /**
   * @brief Focus point
   */
  Vector3 point;
  /**
   * @brief Duty shift
   * @details Duty ratio of ultrasound will be `50% >> duty_shift`. If `duty_shift` is 0, duty ratio is 50%, which means the amplitude is the maximum.
   */
  uint8_t duty_shift;
};

/**
 * @brief FocusSTM is an STM for moving Gain.
 * @details The sampling timing is determined by hardware, thus the sampling time is precise.
 * FocusSTM has following restrictions:
 * - The maximum number of sampling points is 65536.
 * - The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ]/N, where `N` is a 32-bit unsigned integer and must be at
 * 4096.
 */
class FocusSTM final : public STM {
 public:
  explicit FocusSTM(const double freq) : STM(freq, std::nullopt, std::nullopt, std::nullopt) {}

  FocusSTM(const FocusSTM& obj) = default;
  FocusSTM& operator=(const FocusSTM& obj) = default;
  FocusSTM(FocusSTM&& obj) = default;
  FocusSTM& operator=(FocusSTM&& obj) = default;
  ~FocusSTM() override = default;

  static FocusSTM with_sampling_frequency(const double freq) { return FocusSTM(std::nullopt, freq, std::nullopt, std::nullopt); }

  static FocusSTM with_sampling_frequency_division(const uint32_t div) { return FocusSTM(std::nullopt, std::nullopt, div, std::nullopt); }

  template <typename Rep, typename Period>
  static FocusSTM with_sampling_period(const std::chrono::duration<Rep, Period> period) {
    return FocusSTM(std::nullopt, std::nullopt, std::nullopt, std::chrono::duration_cast<std::chrono::nanoseconds>(period));
  }

  [[nodiscard]] native_methods::DatagramPtr ptr(const Geometry&) const override {
    return AUTDSTMFocus(props(), reinterpret_cast<const double*>(_points.data()), _shifts.data(), _shifts.size());
  }

  /**
   * @brief Add focus point
   *
   * @param point Focus point
   * @param duty_shift Duty shift. see [ControlPoint] for details.
   * @return FocusSTM
   */
  void add_focus(Vector3 point, const uint8_t duty_shift = 0) & {
    _points.emplace_back(std::move(point));
    _shifts.emplace_back(duty_shift);
  }

  /**
   * @brief Add focus point
   *
   * @param point Focus point
   * @param duty_shift Duty shift. see [ControlPoint] for details.
   * @return FocusSTM
   */
  [[nodiscard]] FocusSTM&& add_focus(Vector3 point, const uint8_t duty_shift = 0) && {
    _points.emplace_back(std::move(point));
    _shifts.emplace_back(duty_shift);
    return std::move(*this);
  }

  /**
   * @brief Add ControlPoint
   *
   * @param p control point
   * @return FocusSTM
   */
  void add_focus(ControlPoint p) & {
    _points.emplace_back(std::move(p.point));
    _shifts.emplace_back(p.duty_shift);
  }

  /**
   * @brief Add ControlPoint
   *
   * @param p control point
   * @return FocusSTM
   */
  [[nodiscard]] FocusSTM&& add_focus(ControlPoint p) && {
    _points.emplace_back(std::move(p.point));
    _shifts.emplace_back(p.duty_shift);
    return std::move(*this);
  }

  /**
   * @brief Add foci
   *
   * @tparam R
   * @param iter iterator of focus points
   */
  template <std::ranges::viewable_range R>
  auto add_foci_from_iter(R&& iter) -> std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, Vector3>>& {
    for (Vector3 e : iter) {
      _points.emplace_back(std::move(e));
      _shifts.emplace_back(0);
    }
  }

  /**
   * @brief Add foci
   *
   * @tparam R
   * @param iter iterator of focus points
   */
  template <std::ranges::viewable_range R>
  [[nodiscard]] auto add_foci_from_iter(R&& iter) -> std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, Vector3>, FocusSTM&&>&& {
    for (Vector3 e : iter) {
      _points.emplace_back(std::move(e));
      _shifts.emplace_back(0);
    }
    return std::move(*this);
  }

  /**
   * @brief Add foci
   *
   * @tparam R
   * @param iter iterator of [ControlPoint]s
   */
  template <std::ranges::viewable_range R>
  auto add_foci_from_iter(R&& iter) -> std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, ControlPoint>>& {
    for (ControlPoint e : iter) {
      _points.emplace_back(std::move(e.point));
      _shifts.emplace_back(e.duty_shift);
    }
  }

  /**
   * @brief Add foci
   *
   * @tparam R
   * @param iter iterator of [ControlPoint]s
   */
  template <std::ranges::viewable_range R>
  [[nodiscard]] auto add_foci_from_iter(R&& iter) -> std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, ControlPoint>, FocusSTM&&>&& {
    for (ControlPoint e : iter) {
      _points.emplace_back(std::move(e.point));
      _shifts.emplace_back(e.duty_shift);
    }
    return std::move(*this);
  }

  [[nodiscard]] double frequency() const { return frequency_from_size(_points.size()); }

  [[nodiscard]] double sampling_frequency() const { return sampling_frequency_from_size(_points.size()); }

  [[nodiscard]] uint32_t sampling_frequency_division() const { return sampling_frequency_division_from_size(_points.size()); }

  [[nodiscard]] std::chrono::nanoseconds sampling_period() const { return sampling_period_from_size(_points.size()); }

  void with_start_idx(const std::optional<uint16_t> start_idx) & {
    _start_idx = start_idx.has_value() ? static_cast<int32_t>(start_idx.value()) : -1;
  }

  [[nodiscard]] FocusSTM&& with_start_idx(const std::optional<uint16_t> start_idx) && {
    _start_idx = start_idx.has_value() ? static_cast<int32_t>(start_idx.value()) : -1;
    return std::move(*this);
  }

  void with_finish_idx(const std::optional<uint16_t> finish_idx) & {
    _finish_idx = finish_idx.has_value() ? static_cast<int32_t>(finish_idx.value()) : -1;
  }

  [[nodiscard]] FocusSTM&& with_finish_idx(const std::optional<uint16_t> finish_idx) && {
    _finish_idx = finish_idx.has_value() ? static_cast<int32_t>(finish_idx.value()) : -1;
    return std::move(*this);
  }

 private:
  explicit FocusSTM(const std::optional<double> freq, const std::optional<double> sampling_freq, const std::optional<uint32_t> sampling_freq_div,
                    const std::optional<std::chrono::nanoseconds> sampling_period)
      : STM(freq, sampling_freq, sampling_freq_div, sampling_period) {}

  std::vector<Vector3> _points;
  std::vector<uint8_t> _shifts;
};

/**
 * @brief GainSTM is an STM for moving Gain.
 * @details The sampling timing is determined by hardware, thus the sampling time is precise.
 * GainSTM has following restrictions:
 * - The maximum number of sampling Gain is 2048 (Legacy mode) or 1024 (Advanced/AdvancedPhase mode).
 * - The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ]/N, where `N` is a 32-bit unsigned integer and must be at
 * 4096.
 */
class GainSTM final : public STM {
 public:
  /**
   * @brief Constructor
   *
   * @param freq STM frequency
   */
  explicit GainSTM(const double freq) : STM(freq, std::nullopt, std::nullopt, std::nullopt) {}
  GainSTM(const GainSTM& obj) = default;
  GainSTM& operator=(const GainSTM& obj) = default;
  GainSTM(GainSTM&& obj) = default;
  GainSTM& operator=(GainSTM&& obj) = default;
  ~GainSTM() override = default;

  /**
   * @brief Constructor
   *
   * @param freq Sampling frequency
   * @return GainSTM
   */
  static GainSTM with_sampling_frequency(const double freq) { return GainSTM(std::nullopt, freq, std::nullopt, std::nullopt); }

  /**
   * @brief Constructor
   *
   * @param div  Sampling frequency division
   * @return GainSTM
   */
  static GainSTM with_sampling_frequency_division(const uint32_t div) { return GainSTM(std::nullopt, std::nullopt, div, std::nullopt); }

  template <typename Rep, typename Period>
  static GainSTM with_sampling_period(const std::chrono::duration<Rep, Period> period) {
    return GainSTM(std::nullopt, std::nullopt, std::nullopt, std::chrono::duration_cast<std::chrono::nanoseconds>(period));
  }

  [[nodiscard]] native_methods::DatagramPtr ptr(const Geometry& geometry) const override {
    const auto mode = _mode.has_value() ? _mode.value() : native_methods::GainSTMMode::PhaseDutyFull;
    std::vector<native_methods::GainPtr> gains;
    gains.reserve(_gains.size());
    std::ranges::transform(_gains, std::back_inserter(gains), [&](const auto& gain) { return gain->gain_ptr(geometry); });
    return AUTDSTMGain(props(), gains.data(), gains.size(), mode);
  }

  /**
   * @brief Add Gain to the GainSTM
   *
   * @tparam G Gain
   * @param gain gain
   * @return GainSTM
   */
  template <typename G>
  void add_gain(G&& gain) & {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.emplace_back(std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain)));
  }

  /**
   * @brief Add Gain to the GainSTM
   *
   * @tparam G Gain
   * @param gain gain
   * @return GainSTM
   */
  template <typename G>
  [[nodiscard]] GainSTM&& add_gain(G&& gain) && {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.emplace_back(std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain)));
    return std::move(*this);
  }

  /**
   * @brief Add Gains to the GainSTM
   *
   * @tparam R Iterator
   * @param iter gain iterator
   */
  template <std::ranges::viewable_range R>
  auto add_gains_from_iter(R&& iter) -> std::enable_if_t<std::is_base_of_v<Gain, std::remove_reference_t<std::ranges::range_value_t<R>>>>& {
    for (auto e : iter)
      _gains.emplace_back(std::make_shared<std::remove_reference_t<std::ranges::range_value_t<R>>>(std::forward<std::ranges::range_value_t<R>>(e)));
  }

  /**
   * @brief Add Gains to the GainSTM
   *
   * @tparam R Iterator
   * @param iter gain iterator
   * @return GainSTM
   */
  template <std::ranges::viewable_range R>
  auto add_gains_from_iter(R&& iter)
      -> std::enable_if_t<std::is_base_of_v<Gain, std::remove_reference_t<std::ranges::range_value_t<R>>>, GainSTM&&>&& {
    for (auto e : iter)
      _gains.emplace_back(std::make_shared<std::remove_reference_t<std::ranges::range_value_t<R>>>(std::forward<std::ranges::range_value_t<R>>(e)));
    return std::move(*this);
  }

  [[nodiscard]] double frequency() const { return frequency_from_size(_gains.size()); }

  [[nodiscard]] double sampling_frequency() const { return sampling_frequency_from_size(_gains.size()); }

  [[nodiscard]] uint32_t sampling_frequency_division() const { return sampling_frequency_division_from_size(_gains.size()); }

  [[nodiscard]] std::chrono::nanoseconds sampling_period() const { return sampling_period_from_size(_gains.size()); }

  void with_mode(const native_methods::GainSTMMode mode) & { _mode = mode; }
  [[nodiscard]] GainSTM&& with_mode(const native_methods::GainSTMMode mode) && {
    _mode = mode;
    return std::move(*this);
  }

  void with_start_idx(const std::optional<uint16_t> start_idx) & {
    _start_idx = start_idx.has_value() ? static_cast<int32_t>(start_idx.value()) : -1;
  }

  [[nodiscard]] GainSTM&& with_start_idx(const std::optional<uint16_t> start_idx) && {
    _start_idx = start_idx.has_value() ? static_cast<int32_t>(start_idx.value()) : -1;
    return std::move(*this);
  }

  void with_finish_idx(const std::optional<uint16_t> finish_idx) & {
    _finish_idx = finish_idx.has_value() ? static_cast<int32_t>(finish_idx.value()) : -1;
  }

  [[nodiscard]] GainSTM&& with_finish_idx(const std::optional<uint16_t> finish_idx) && {
    _finish_idx = finish_idx.has_value() ? static_cast<int32_t>(finish_idx.value()) : -1;
    return std::move(*this);
  }

 private:
  explicit GainSTM(const std::optional<double> freq, const std::optional<double> sampling_freq, const std::optional<uint32_t> sampling_freq_div,
                   const std::optional<std::chrono::nanoseconds> sampling_period)
      : STM(freq, sampling_freq, sampling_freq_div, sampling_period) {}

  std::vector<std::shared_ptr<Gain>> _gains;
  std::optional<native_methods::GainSTMMode> _mode;
};

}  // namespace autd3::internal
