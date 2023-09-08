// File: primitive.hpp
// Project: modulation
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <numeric>

#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

/**
 * @brief Modulation to cache the result of calculation
 */
class Cache : public internal::Modulation {
 public:
  template <class M>
  Cache(M&& m) : _freq_div(m.sampling_frequency_division()) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
    char err[256]{};
    const auto size = internal::native_methods::AUTDModulationSize(m.modulation_ptr(), err);
    if (size == internal::native_methods::AUTD3_ERR) throw internal::AUTDException(err);
    _buffer.resize(static_cast<size_t>(size));
    if (internal::native_methods::AUTDModulationCalc(m.modulation_ptr(), _buffer.data(), err) == internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationCustom(_freq_div, _buffer.data(), _buffer.size());
  }

  const std::vector<double>& buffer() const { return _buffer; }
  std::vector<double>& buffer() { return _buffer; }

  [[nodiscard]] std::vector<double>::const_iterator begin() const noexcept { return _buffer.cbegin(); }
  [[nodiscard]] std::vector<double>::const_iterator end() const noexcept { return _buffer.cend(); }
  [[nodiscard]] std::vector<double>::iterator begin() noexcept { return _buffer.begin(); }
  [[nodiscard]] std::vector<double>::iterator end() noexcept { return _buffer.end(); }
  [[nodiscard]] const double& operator[](const size_t i) const { return _buffer[i]; }
  [[nodiscard]] double& operator[](const size_t i) { return _buffer[i]; }

 private:
  std::vector<double> _buffer;
  uint32_t _freq_div;
};

#define AUTD3_IMPL_WITH_CACHE_MODULATION                                 \
  [[nodiscard]] Cache with_cache()&& { return Cache(std::move(*this)); } \
  [[nodiscard]] Cache with_cache()& { return Cache(*this); }

/**
 * @brief Modulation for modulating radiation pressure
 */
class RadiationPressure : public internal::Modulation {
 public:
  template <class M>
  RadiationPressure(M&& m) : _freq_div(m.sampling_frequency_division()) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
    char err[256]{};
    const auto size = internal::native_methods::AUTDModulationSize(m.modulation_ptr(), err);
    if (size == internal::native_methods::AUTD3_ERR) throw internal::AUTDException(err);
    _buffer.resize(static_cast<size_t>(size));
    if (internal::native_methods::AUTDModulationCalc(m.modulation_ptr(), _buffer.data(), err) == internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
    std::transform(_buffer.begin(), _buffer.end(), _buffer.begin(), [](const double v) { return std::sqrt(v); });
  }

  AUTD3_IMPL_WITH_CACHE_MODULATION

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationCustom(_freq_div, _buffer.data(), _buffer.size());
  }

 private:
  std::vector<double> _buffer;
  uint32_t _freq_div;
};

#define AUTD3_IMPL_WITH_RADIATION_PRESSURE                                                                    \
  [[nodiscard]] RadiationPressure with_radiation_pressure()&& { return RadiationPressure(std::move(*this)); } \
  [[nodiscard]] RadiationPressure with_radiation_pressure()& { return RadiationPressure(*this); }

/**
 * @brief Without modulation
 */
class Static final : public internal::Modulation {
 public:
  Static() = default;

  AUTD3_IMPL_WITH_CACHE_MODULATION
  AUTD3_IMPL_WITH_RADIATION_PRESSURE

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (0.0 - 1.0)
   * @return Static
   */
  void with_amp(const double amp) & { _amp = amp; }

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (0.0 - 1.0)
   * @return Static
   */
  [[nodiscard]] Static&& with_amp(const double amp) && {
    _amp = amp;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = internal::native_methods::AUTDModulationStatic();
    if (_amp.has_value()) ptr = AUTDModulationStaticWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  std::optional<double> _amp;
};

/**
 * @brief Sine wave modulation
 */
class Sine final : public internal::Modulation {
 public:
  /**
   * @brief Constructor.
   * @details The sine wave is defined as `amp / 2 * sin(2π * freq * t + phase) + offset`, where `t` is time, and `amp = 1`, `offset
   * = 0.5` by default.
   *
   * @param freq Frequency of sine wave
   */
  explicit Sine(const int32_t freq) : _freq(freq) {}

  AUTD3_IMPL_WITH_CACHE_MODULATION
  AUTD3_IMPL_WITH_RADIATION_PRESSURE

  /**
   * @brief Set amplitude
   *
   * @param amp peek to peek amplitude of sine wave
   * @return Sine
   */
  void with_amp(const double amp) & { _amp = amp; }

  /**
   * @brief Set amplitude
   *
   * @param amp peek to peek amplitude of sine wave
   * @return Sine
   */
  [[nodiscard]] Sine&& with_amp(const double amp) && {
    _amp = amp;
    return std::move(*this);
  }

  /**
   * @brief Set phase
   *
   * @param phase Phase of sine wave
   * @return Sine
   */
  void with_phase(const double phase) & { _phase = phase; }

  /**
   * @brief Set phase
   *
   * @param phase Phase of sine wave
   * @return Sine
   */
  [[nodiscard]] Sine&& with_phase(const double phase) && {
    _phase = phase;
    return std::move(*this);
  }

  /**
   * @brief Set offset
   *
   * @param offset Offset of sine wave
   * @return Sine
   */
  void with_offset(const double offset) & { _offset = offset; }

  /**
   * @brief Set offset
   *
   * @param offset Offset of sine wave
   * @return Sine
   */
  [[nodiscard]] Sine&& with_offset(const double offset) && {
    _offset = offset;
    return std::move(*this);
  }

  /**
   * @brief Set sampling frequency division
   * @details The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ] / div.
   */
  void with_sampling_frequency_division(const uint32_t div) & { _freq_div = div; }

  /**
   * @brief Set sampling frequency division
   * @details The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ] / div.
   */
  [[nodiscard]] Sine&& with_sampling_frequency_division(const uint32_t div) && {
    _freq_div = div;
    return std::move(*this);
  }

  /**
   * @brief Set sampling frequency
   */
  void with_sampling_frequency(const double freq) & {
    with_sampling_frequency_division(static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }

  /**
   * @brief Set sampling frequency
   */
  [[nodiscard]] Sine&& with_sampling_frequency(const double freq) && {
    return std::move(*this).with_sampling_frequency_division(
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = internal::native_methods::AUTDModulationSine(_freq);
    if (_amp.has_value()) ptr = AUTDModulationSineWithAmp(ptr, _amp.value());
    if (_phase.has_value()) ptr = AUTDModulationSineWithPhase(ptr, _phase.value());
    if (_offset.has_value()) ptr = AUTDModulationSineWithOffset(ptr, _offset.value());
    if (_freq_div.has_value()) ptr = AUTDModulationSineWithSamplingFrequencyDivision(ptr, _freq_div.value());
    return ptr;
  }

 private:
  int32_t _freq;
  std::optional<double> _amp;
  std::optional<double> _phase;
  std::optional<double> _offset;
  std::optional<uint32_t> _freq_div;
};

/**
 * @brief Multi-frequency sine wave modulation
 */
class Fourier final : public internal::Modulation {
 public:
  Fourier() = default;

  AUTD3_IMPL_WITH_CACHE_MODULATION
  AUTD3_IMPL_WITH_RADIATION_PRESSURE

  void add_component(Sine component) & { _components.emplace_back(component); }

  [[nodiscard]] Fourier&& add_component(Sine component) && {
    _components.emplace_back(component);
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return std::accumulate(
        _components.begin(), _components.end(), internal::native_methods::AUTDModulationFourier(),
        [](internal::native_methods::ModulationPtr ptr, const Sine& sine) { return AUTDModulationFourierAddComponent(ptr, sine.modulation_ptr()); });
  }

 private:
  std::vector<Sine> _components;
};

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
  AUTD3_IMPL_WITH_RADIATION_PRESSURE

  /**
   * @brief Set amplitude
   *
   * @param amp peek to peek amplitude of sine wave
   * @return Sine
   */
  void with_amp(const double amp) & { _amp = amp; }

  /**
   * @brief Set amplitude
   *
   * @param amp peek to peek amplitude of sine wave
   * @return Sine
   */
  [[nodiscard]] SineLegacy&& with_amp(const double amp) && {
    _amp = amp;
    return std::move(*this);
  }

  /**
   * @brief Set offset
   *
   * @param offset Offset of sine wave
   * @return Sine
   */
  void with_offset(const double offset) & { _offset = offset; }

  /**
   * @brief Set offset
   *
   * @param offset Offset of sine wave
   * @return Sine
   */
  [[nodiscard]] SineLegacy&& with_offset(const double offset) && {
    _offset = offset;
    return std::move(*this);
  }

  /**
   * @brief Set sampling frequency division
   * @details The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ] / div.
   */
  void with_sampling_frequency_division(const uint32_t div) & { _freq_div = div; }

  /**
   * @brief Set sampling frequency division
   * @details The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ] / div.
   */
  [[nodiscard]] SineLegacy&& with_sampling_frequency_division(const uint32_t div) && {
    _freq_div = div;
    return std::move(*this);
  }

  /**
   * @brief Set sampling frequency
   */
  void with_sampling_frequency(const double freq) & {
    with_sampling_frequency_division(static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }

  /**
   * @brief Set sampling frequency
   */
  [[nodiscard]] SineLegacy&& with_sampling_frequency(const double freq) && {
    return std::move(*this).with_sampling_frequency_division(
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }

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

/**
 * @brief Square wave modulation
 */
class Square final : public internal::Modulation {
 public:
  /**
   * @brief Constructor
   *
   * @param freq Frequency of square wave
   */
  explicit Square(const int32_t freq) : _freq(freq) {}

  AUTD3_IMPL_WITH_CACHE_MODULATION
  AUTD3_IMPL_WITH_RADIATION_PRESSURE

  /**
   * @brief set low level amplitude
   *
   * @param low low level amplitude (0.0 - 1.0)
   * @return Square
   */
  void with_low(const double low) & { _low = low; }

  /**
   * @brief set low level amplitude
   *
   * @param low low level amplitude (0.0 - 1.0)
   * @return Square
   */
  [[nodiscard]] Square&& with_low(const double low) && {
    _low = low;
    return std::move(*this);
  }

  /**
   * @brief set high level amplitude
   *
   * @param high high level amplitude (0.0 - 1.0)
   * @return Square
   */
  void with_high(const double high) & { _high = high; }

  /**
   * @brief set high level amplitude
   *
   * @param high high level amplitude (0.0 - 1.0)
   * @return Square
   */
  [[nodiscard]] Square&& with_high(const double high) && {
    _high = high;
    return std::move(*this);
  }

  /**
   * @brief set duty ratio.
   * @details Duty ratio is defined as `Th / (Th + Tl)`, where `Th` is high level duration, and `Tl` is low level duration.
   *
   * @param duty duty ratio (0.0 - 1.0)
   * @return Square
   */
  void with_duty(const double duty) & { _duty = duty; }

  /**
   * @brief set duty ratio.
   * @details Duty ratio is defined as `Th / (Th + Tl)`, where `Th` is high level duration, and `Tl` is low level duration.
   *
   * @param duty duty ratio (0.0 - 1.0)
   * @return Square
   */
  [[nodiscard]] Square&& with_duty(const double duty) && {
    _duty = duty;
    return std::move(*this);
  }

  /**
   * @brief Set sampling frequency division
   * @details The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ] / div.
   */
  void with_sampling_frequency_division(const uint32_t div) & { _freq_div = div; }

  /**
   * @brief Set sampling frequency division
   * @details The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ] / div.
   */
  [[nodiscard]] Square&& with_sampling_frequency_division(const uint32_t div) && {
    _freq_div = div;
    return std::move(*this);
  }

  /**
   * @brief Set sampling frequency
   */
  void with_sampling_frequency(const double freq) & {
    with_sampling_frequency_division(static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }

  /**
   * @brief Set sampling frequency
   */
  [[nodiscard]] Square&& with_sampling_frequency(const double freq) && {
    return std::move(*this).with_sampling_frequency_division(
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }

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

/**
 * @brief Base class for custom modulation
 */
class Modulation : public internal::Modulation {
 public:
  explicit Modulation(const double sampling_freq)
      : _freq_div(static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / sampling_freq)) {}
  explicit Modulation(const uint32_t freq_div) : _freq_div(freq_div) {}

  [[nodiscard]] virtual std::vector<double> calc() const = 0;

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    const auto buffer = calc();
    const auto size = buffer.size();
    return internal::native_methods::AUTDModulationCustom(_freq_div, buffer.data(), size);
  }

 private:
  uint32_t _freq_div;
};

}  // namespace autd3::modulation
