// File: primitive.hpp
// Project: modulation
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
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
  explicit Static(const driver::autd3_float_t amp = 1.0) noexcept : Modulation(), _amp(amp) {}

  bool calc() override {
    this->_buffer.resize(2, 0);
    for (size_t i = 0; i < 2; i++) {
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp<driver::autd3_float_t>(_amp, 0, 1)) / driver::pi * 510));
      this->_buffer.at(i) = duty;
    }
    return true;
  }

  ~Static() override = default;
  Static(const Static& v) noexcept = delete;
  Static& operator=(const Static& obj) = delete;
  Static(Static&& obj) = default;
  Static& operator=(Static&& obj) = default;

 private:
  driver::autd3_float_t _amp;
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
  explicit Sine(const int freq, const driver::autd3_float_t amp = 1.0, const driver::autd3_float_t offset = 0.5) noexcept
      : Modulation(), _freq(freq), _amp(amp), _offset(offset) {}

  bool calc() override {
    const auto f_s = static_cast<int32_t>(sampling_frequency());

    const auto f = std::clamp(this->_freq, 1, f_s / 2);

    const auto k = std::gcd(f_s, f);

    const size_t n = f_s / k;
    const size_t d = f / k;

    this->_buffer.resize(n, 0);
    for (size_t i = 0; i < n; i++) {
      const auto amp = this->_amp / 2 * std::sin(2 * driver::pi * static_cast<driver::autd3_float_t>(d * i) / static_cast<driver::autd3_float_t>(n)) +
                       this->_offset;
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp<driver::autd3_float_t>(amp, 0, 1)) / driver::pi * 510));
      this->_buffer.at(i) = duty;
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
  driver::autd3_float_t _amp;
  driver::autd3_float_t _offset;
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
  explicit SineSquared(const int freq, const driver::autd3_float_t amp = 1.0, const driver::autd3_float_t offset = 0.5) noexcept
      : Modulation(), _freq(freq), _amp(amp), _offset(offset) {}

  bool calc() override {
    const auto f_s = static_cast<int32_t>(sampling_frequency());

    const auto f = std::clamp(this->_freq, 1, f_s / 2);

    const auto k = std::gcd(f_s, f);

    const size_t n = f_s / k;
    const size_t d = f / k;

    this->_buffer.resize(n, 0);
    for (size_t i = 0; i < n; i++) {
      const auto amp =
          std::sqrt(this->_amp / 2 * std::sin(2 * driver::pi * static_cast<driver::autd3_float_t>(d * i) / static_cast<driver::autd3_float_t>(n)) +
                    this->_offset);
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp<driver::autd3_float_t>(amp, 0, 1)) / driver::pi * 510));
      this->_buffer.at(i) = duty;
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
  driver::autd3_float_t _amp;
  driver::autd3_float_t _offset;
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
  explicit SineLegacy(const driver::autd3_float_t freq, const driver::autd3_float_t amp = 1.0, const driver::autd3_float_t offset = 0.5) noexcept
      : Modulation(), _freq(freq), _amp(amp), _offset(offset) {}

  bool calc() override {
    const auto f_s = sampling_frequency();
    const auto f = (std::min)(this->_freq, f_s / 2);

    const auto t = static_cast<size_t>(std::round(f_s / f));
    this->_buffer.resize(t, 0);
    for (size_t i = 0; i < t; i++) {
      driver::autd3_float_t amp =
          _offset + _amp * std::cos(2 * driver::pi * static_cast<driver::autd3_float_t>(i) / static_cast<driver::autd3_float_t>(t)) / 2;
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp<driver::autd3_float_t>(amp, 0, 1)) / driver::pi * 510));
      this->_buffer.at(i) = duty;
    }
    return true;
  }

 private:
  driver::autd3_float_t _freq;
  driver::autd3_float_t _amp;
  driver::autd3_float_t _offset;
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
  explicit Square(const int freq, const driver::autd3_float_t low = 0.0, const driver::autd3_float_t high = 1.0,
                  const driver::autd3_float_t duty = 0.5)
      : _freq(freq), _low(low), _high(high), _duty(duty) {}

  bool calc() override {
    const auto f_s = static_cast<int32_t>(sampling_frequency());
    const auto f = std::clamp(this->_freq, 1, f_s / 2);
    const auto k = std::gcd(f_s, f);
    const size_t n = f_s / k;
    const size_t d = f / k;

    const auto low = static_cast<uint8_t>(std::round(std::asin(std::clamp<driver::autd3_float_t>(_low, 0, 1)) / driver::pi * 510));
    std::fill(this->_buffer.begin(), this->_buffer.end(), low);
    this->_buffer.resize(n, low);

    const auto high = static_cast<uint8_t>(std::round(std::asin(std::clamp<driver::autd3_float_t>(_high, 0, 1)) / driver::pi * 510));
    auto* cursor = this->_buffer.data();
    for (size_t i = 0; i < d; i++) {
      const size_t size = (n + i) / d;
      std::memset(cursor, high, static_cast<size_t>(std::round(static_cast<driver::autd3_float_t>(size) * _duty)));
      cursor += size;
    }
    return true;
  }

 private:
  int _freq;
  driver::autd3_float_t _low;
  driver::autd3_float_t _high;
  driver::autd3_float_t _duty;
};

}  // namespace autd3::modulation
