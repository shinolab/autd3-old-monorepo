// File: primitive.hpp
// Project: modulation
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/modulation.hpp"
#include "autd3/driver/defined.hpp"

namespace autd3::modulation {

/**
 * @brief Static (Without modulation)
 */
class Static final : public core::Modulation {
 public:
  /**
   * @param[in] amp amplitude
   */
  explicit Static(const double amp = 1.0) noexcept : Modulation(), _amp(amp) {}

  bool calc() override {
    this->_props.buffer.resize(2, 0);
    for (size_t i = 0; i < 2; i++) {
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp(_amp, 0.0, 1.0)) / driver::pi * 510.0));
      this->_props.buffer.at(i) = duty;
    }
    return true;
  }

  ~Static() override = default;
  Static(const Static& v) noexcept = delete;
  Static& operator=(const Static& obj) = delete;
  Static(Static&& obj) = default;
  Static& operator=(Static&& obj) = default;

 private:
  double _amp;
};

/**
 * @brief Sine wave modulation in ultrasound amplitude
 */
class Sine final : public core::Modulation {
 public:
  /**
   * @param[in] freq Frequency of the sine wave
   * @param[in] amp peek to peek ultrasound amplitude (Maximum value is 1.0)
   * @param[in] offset offset of ultrasound amplitude
   * @details Ultrasound amplitude oscillate from offset-amp/2 to offset+amp/2.
   * If the value exceeds the range of [0, 1], the value will be clamped in the [0, 1].
   */
  explicit Sine(const int freq, const double amp = 1.0, const double offset = 0.5) noexcept : Modulation(), _freq(freq), _amp(amp), _offset(offset) {}

  bool calc() override {
    const auto f_s = static_cast<int32_t>(sampling_frequency());

    const auto f = std::clamp(this->_freq, 1, f_s / 2);

    const auto k = std::gcd(f_s, f);

    const size_t n = f_s / k;
    const size_t d = f / k;

    this->_props.buffer.resize(n, 0);
    for (size_t i = 0; i < n; i++) {
      const auto amp = this->_amp / 2.0 * std::sin(2.0 * driver::pi * static_cast<double>(d * i) / static_cast<double>(n)) + this->_offset;
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp(amp, 0.0, 1.0)) / driver::pi * 510.0));
      this->_props.buffer.at(i) = duty;
    }
    return true;
  }

  ~Sine() override = default;
  Sine(const Sine& v) noexcept = delete;
  Sine& operator=(const Sine& obj) = delete;
  Sine(Sine&& obj) = default;
  Sine& operator=(Sine&& obj) = default;

 private:
  int _freq;
  double _amp;
  double _offset;
};

/**
 * @brief Sine wave modulation in squared acoustic pressure, which is proportional to radiation pressure
 */
class SineSquared final : public core::Modulation {
 public:
  /**
   * @param[in] freq Frequency of the sine wave
   * @param[in] amp peek to peek amplitude of radiation pressure (Maximum value is 1.0)
   * @param[in] offset offset of radiation pressure
   * @details Radiation pressure oscillate from offset-amp/2 to offset+amp/2
   * If the value exceeds the range of [0, 1], the value will be clamped in the [0, 1].
   */
  explicit SineSquared(const int freq, const double amp = 1.0, const double offset = 0.5) noexcept
      : Modulation(), _freq(freq), _amp(amp), _offset(offset) {}

  bool calc() override {
    const auto f_s = static_cast<int32_t>(sampling_frequency());

    const auto f = std::clamp(this->_freq, 1, f_s / 2);

    const auto k = std::gcd(f_s, f);

    const size_t n = f_s / k;
    const size_t d = f / k;

    this->_props.buffer.resize(n, 0);
    for (size_t i = 0; i < n; i++) {
      const auto amp = std::sqrt(this->_amp / 2.0 * std::sin(2.0 * driver::pi * static_cast<double>(d * i) / static_cast<double>(n)) + this->_offset);
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp(amp, 0.0, 1.0)) / driver::pi * 510.0));
      this->_props.buffer.at(i) = duty;
    }
    return true;
  }

  ~SineSquared() override = default;
  SineSquared(const SineSquared& v) noexcept = delete;
  SineSquared& operator=(const SineSquared& obj) = delete;
  SineSquared(SineSquared&& obj) = default;
  SineSquared& operator=(SineSquared&& obj) = default;

 private:
  int _freq;
  double _amp;
  double _offset;
};

/**
 * @brief Sine wave modulation in ultrasound amplitude (Legacy)
 */
class SineLegacy final : public core::Modulation {
 public:
  /**
   * @param[in] freq Frequency of the sine wave
   * @param[in] amp peek to peek ultrasound amplitude (Maximum value is 1.0)
   * @param[in] offset offset of ultrasound amplitude
   * @details Ultrasound amplitude oscillate from offset-amp/2 to offset+amp/2.
   * If the value exceeds the range of [0, 1], the value will be clamped in the [0, 1].
   */
  explicit SineLegacy(const double freq, const double amp = 1.0, const double offset = 0.5) noexcept
      : Modulation(), _freq(freq), _amp(amp), _offset(offset) {}

  bool calc() override {
    const auto f_s = sampling_frequency();
    const auto f = (std::min)(this->_freq, f_s / 2.0);

    const auto t = static_cast<size_t>(std::round(f_s / f));
    this->_props.buffer.resize(t, 0);
    for (size_t i = 0; i < t; i++) {
      double amp = _offset + 0.5 * _amp * std::cos(2.0 * driver::pi * static_cast<double>(i) / static_cast<double>(t));
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp(amp, 0.0, 1.0)) / driver::pi * 510.0));
      this->_props.buffer.at(i) = duty;
    }
    return true;
  }

 private:
  double _freq;
  double _amp;
  double _offset;
};

/**
 * @brief Square wave modulation
 */
class Square final : public core::Modulation {
 public:
  /**
   * @param[in] freq Frequency of the square wave
   * @param[in] low low level in amplitude (0 to 1)
   * @param[in] high high level in amplitude (0 to 1)
   * @param[in] duty duty ratio of square wave
   */
  Square(const int freq, const double low = 0.0, const double high = 1.0, const double duty = 0.5)
      : _freq(freq), _low(low), _high(high), _duty(duty) {}

  bool calc() override {
    const auto f_s = static_cast<int32_t>(sampling_frequency());
    const auto f = std::clamp(this->_freq, 1, f_s / 2);
    const auto k = std::gcd(f_s, f);
    const size_t n = f_s / k;
    const size_t d = f / k;

    const auto low = static_cast<uint8_t>(std::round(std::asin(std::clamp(_low, 0.0, 1.0)) / driver::pi * 510.0));
    std::fill(this->_props.buffer.begin(), this->_props.buffer.end(), low);
    this->_props.buffer.resize(n, low);

    const auto high = static_cast<uint8_t>(std::round(std::asin(std::clamp(_high, 0.0, 1.0)) / driver::pi * 510.0));
    auto* cursor = this->_props.buffer.data();
    for (size_t i = 0; i < d; i++) {
      const size_t size = (n + i) / d;
      std::memset(cursor, high, static_cast<size_t>(std::round(static_cast<double>(size) * _duty)));
      cursor += size;
    }
    return true;
  }

 private:
  int _freq;
  double _low;
  double _high;
  double _duty;
};

}  // namespace autd3::modulation
