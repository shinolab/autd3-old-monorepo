// File: stm.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
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
#include "autd3/internal/sampling_config.hpp"
#include "emit_intensity.hpp"

namespace autd3::internal {

class STM {
 public:
  explicit STM(const std::optional<double> freq, const std::optional<std::chrono::nanoseconds> period,
               const std::optional<SamplingConfiguration> config)
      : _freq(freq), _period(period), _config(config) {}

  STM(const STM& obj) = default;
  STM& operator=(const STM& obj) = default;
  STM(STM&& obj) = default;
  STM& operator=(STM&& obj) = default;
  virtual ~STM() = default;  // LCOV_EXCL_LINE

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
    if (_freq.has_value()) ptr = native_methods::AUTDSTMPropsNew(_freq.value());
    if (_period.has_value()) ptr = native_methods::AUTDSTMPropsFromPeriod(static_cast<uint64_t>(_period.value().count()));
    if (_config.has_value()) ptr = AUTDSTMPropsFromSamplingConfig(static_cast<native_methods::SamplingConfiguration>(_config.value()));
    if (ptr._0 == nullptr) throw std::runtime_error("unreachable!");
    ptr = AUTDSTMPropsWithStartIdx(ptr, _start_idx);
    ptr = AUTDSTMPropsWithFinishIdx(ptr, _finish_idx);
    return ptr;
  }

  [[nodiscard]] double frequency_from_size(const size_t size) const { return AUTDSTMPropsFrequency(props(), size); }

  [[nodiscard]] SamplingConfiguration sampling_config_from_size(const size_t size) const {
    return SamplingConfiguration(validate(AUTDSTMPropsSamplingConfig(props(), size)));
  }

  [[nodiscard]] std::chrono::nanoseconds period_from_size(const size_t size) const {
    return std::chrono::nanoseconds(AUTDSTMPropsPeriod(props(), size));
  }

  std::optional<double> _freq;
  std::optional<std::chrono::nanoseconds> _period;
  std::optional<SamplingConfiguration> _config;
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
   * @details Duty ratio of ultrasound will be `50% >> duty_shift`. If
   * `duty_shift` is 0, duty ratio is 50%, which means the amplitude is the
   * maximum.
   */
  EmitIntensity intensity;
};

template <class R>
concept focus_range_v = std::ranges::viewable_range<R> && std::same_as<std::ranges::range_value_t<R>, Vector3>;

template <class R>
concept focus_range_c = std::ranges::viewable_range<R> && std::same_as<std::ranges::range_value_t<R>, ControlPoint>;

/**
 * @brief FocusSTM is an STM for moving Gain.
 * @details The sampling timing is determined by hardware, thus the sampling
 * time is precise. FocusSTM has following restrictions:
 * - The maximum number of sampling points is 65536.
 * - The sampling frequency is
 * [autd3::internal::native_methods::FPGA_CLK_FREQ]/N, where `N` is a 32-bit
 * unsigned integer and must be at 4096.
 */
class FocusSTM final : public STM {
 public:
  explicit FocusSTM(const double freq) : STM(freq, std::nullopt, std::nullopt) {}

  FocusSTM(const FocusSTM& obj) = default;
  FocusSTM& operator=(const FocusSTM& obj) = default;
  FocusSTM(FocusSTM&& obj) = default;
  FocusSTM& operator=(FocusSTM&& obj) = default;
  ~FocusSTM() override = default;  // LCOV_EXCL_LINE

  static FocusSTM from_sampling_config(const SamplingConfiguration config) { return FocusSTM(std::nullopt, std::nullopt, config); }

  template <typename Rep, typename Period>
  static FocusSTM from_period(const std::chrono::duration<Rep, Period> period) {
    return FocusSTM(std::nullopt, std::chrono::duration_cast<std::chrono::nanoseconds>(period), std::nullopt);
  }

  [[nodiscard]] native_methods::DatagramPtr ptr(const geometry::Geometry&) const {
    return validate(AUTDSTMFocus(props(), reinterpret_cast<const double*>(_points.data()), reinterpret_cast<const uint8_t*>(_intensities.data()),
                                 _intensities.size()));
  }

  /**
   * @brief Add focus point
   *
   * @param point Focus point
   * @param intensity Emission intensity
   * @return FocusSTM
   */
  void add_focus(Vector3 point, const EmitIntensity intensity = EmitIntensity::maximum()) & {
    _points.emplace_back(std::move(point));
    _intensities.emplace_back(intensity);
  }

  /**
   * @brief Add focus point
   *
   * @param point Focus point
   * @param intensity Emission intensity
   * @return FocusSTM
   */
  [[nodiscard]] FocusSTM&& add_focus(Vector3 point, const EmitIntensity intensity = EmitIntensity::maximum()) && {
    _points.emplace_back(std::move(point));
    _intensities.emplace_back(intensity);
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
    _intensities.emplace_back(p.intensity);
  }

  /**
   * @brief Add ControlPoint
   *
   * @param p control point
   * @return FocusSTM
   */
  [[nodiscard]] FocusSTM&& add_focus(ControlPoint p) && {
    _points.emplace_back(std::move(p.point));
    _intensities.emplace_back(p.intensity);
    return std::move(*this);
  }

  /**
   * @brief Add foci
   *
   * @tparam R
   * @param iter iterator of focus points
   */
  template <focus_range_v R>
  void add_foci_from_iter(R&& iter) & {
    for (Vector3 e : iter) {
      _points.emplace_back(std::move(e));
      _intensities.emplace_back(EmitIntensity::maximum());
    }
  }

  /**
   * @brief Add foci
   *
   * @tparam R
   * @param iter iterator of focus points
   */
  template <focus_range_v R>
  [[nodiscard]] FocusSTM add_foci_from_iter(R&& iter) && {
    for (Vector3 e : iter) {
      _points.emplace_back(std::move(e));
      _intensities.emplace_back(EmitIntensity::maximum());
    }
    return std::move(*this);
  }

  /**
   * @brief Add foci
   *
   * @tparam R
   * @param iter iterator of [ControlPoint]s
   */
  template <focus_range_c R>
  void add_foci_from_iter(R&& iter) & {
    for (ControlPoint e : iter) {
      _points.emplace_back(std::move(e.point));
      _intensities.emplace_back(e.intensity);
    }
  }

  /**
   * @brief Add foci
   *
   * @tparam R
   * @param iter iterator of [ControlPoint]s
   */
  template <focus_range_c R>
  [[nodiscard]] FocusSTM add_foci_from_iter(R&& iter) && {
    for (ControlPoint e : iter) {
      _points.emplace_back(std::move(e.point));
      _intensities.emplace_back(e.intensity);
    }
    return std::move(*this);
  }

  [[nodiscard]] double frequency() const { return frequency_from_size(_points.size()); }
  [[nodiscard]] std::chrono::nanoseconds period() const { return period_from_size(_points.size()); }
  [[nodiscard]] SamplingConfiguration sampling_config() const { return sampling_config_from_size(_points.size()); }

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
  explicit FocusSTM(const std::optional<double> freq, const std::optional<std::chrono::nanoseconds> period,
                    const std::optional<SamplingConfiguration> config)
      : STM(freq, period, config) {}

  std::vector<Vector3> _points;
  std::vector<EmitIntensity> _intensities;
};

template <class R>
concept gain_range = std::ranges::viewable_range<R> && gain<std::ranges::range_value_t<R>>;

/**
 * @brief GainSTM is an STM for moving Gain.
 * @details The sampling timing is determined by hardware, thus the sampling
 * time is precise. GainSTM has following restrictions:
 * - The maximum number of sampling Gain is 2048.
 * - The sampling frequency is
 * [autd3::internal::native_methods::FPGA_CLK_FREQ]/N, where `N` is a 32-bit
 * unsigned integer and must be at 4096.
 */
class GainSTM final : public STM {
 public:
  /**
   * @brief Constructor
   *
   * @param freq STM frequency
   */
  explicit GainSTM(const double freq) : STM(freq, std::nullopt, std::nullopt) {}
  GainSTM(const GainSTM& obj) = default;
  GainSTM& operator=(const GainSTM& obj) = default;
  GainSTM(GainSTM&& obj) = default;
  GainSTM& operator=(GainSTM&& obj) = default;
  ~GainSTM() override = default;  // LCOV_EXCL_LINE

  /**
   * @brief Constructor
   *
   * @param config Sampling configuration
   * @return GainSTM
   */
  static GainSTM from_sampling_config(const SamplingConfiguration config) { return GainSTM(std::nullopt, std::nullopt, config); }

  template <typename Rep, typename Period>
  static GainSTM from_period(const std::chrono::duration<Rep, Period> period) {
    return GainSTM(std::nullopt, std::nullopt, std::nullopt, std::chrono::duration_cast<std::chrono::nanoseconds>(period));
  }

  [[nodiscard]] native_methods::DatagramPtr ptr(const geometry::Geometry& geometry) const {
    const auto mode = _mode.has_value() ? _mode.value() : native_methods::GainSTMMode::PhaseIntensityFull;
    std::vector<native_methods::GainPtr> gains;
    gains.reserve(_gains.size());
    std::ranges::transform(_gains, std::back_inserter(gains), [&](const auto& gain) { return gain->gain_ptr(geometry); });
    return validate(AUTDSTMGain(props(), gains.data(), static_cast<uint32_t>(gains.size()), mode));
  }

  /**
   * @brief Add Gain to the GainSTM
   *
   * @tparam G Gain
   * @param gain gain
   * @return GainSTM
   */
  template <gain G>
  void add_gain(G&& gain) & {
    _gains.emplace_back(std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain)));
  }

  /**
   * @brief Add Gain to the GainSTM
   *
   * @tparam G Gain
   * @param gain gain
   * @return GainSTM
   */
  template <gain G>
  [[nodiscard]] GainSTM&& add_gain(G&& gain) && {
    _gains.emplace_back(std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain)));
    return std::move(*this);
  }

  /**
   * @brief Add Gains to the GainSTM
   *
   * @tparam R Iterator
   * @param iter gain iterator
   */
  template <gain_range R>
  void add_gains_from_iter(R&& iter) & {
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
  template <gain_range R>
  GainSTM add_gains_from_iter(R&& iter) && {
    for (auto e : iter)
      _gains.emplace_back(std::make_shared<std::remove_reference_t<std::ranges::range_value_t<R>>>(std::forward<std::ranges::range_value_t<R>>(e)));
    return std::move(*this);
  }

  [[nodiscard]] double frequency() const { return frequency_from_size(_gains.size()); }
  [[nodiscard]] std::chrono::nanoseconds period() const { return period_from_size(_gains.size()); }
  [[nodiscard]] SamplingConfiguration sampling_config() const { return sampling_config_from_size(_gains.size()); }

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
  explicit GainSTM(const std::optional<double> freq, const std::optional<std::chrono::nanoseconds> period,
                   const std::optional<SamplingConfiguration> config)
      : STM(freq, period, config) {}

  std::vector<std::shared_ptr<Gain>> _gains;
  std::optional<native_methods::GainSTMMode> _mode;
};

}  // namespace autd3::internal
