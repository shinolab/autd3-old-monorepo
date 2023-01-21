// File: primitive.hpp
// Project: modulation
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <utility>
#include <vector>

#include "autd3/core/modulation.hpp"
#include "autd3/driver/defined.hpp"

namespace autd3::modulation {

inline uint8_t to_duty(const driver::autd3_float_t amp) {
  return static_cast<uint8_t>(std::round(std::asin(std::clamp<driver::autd3_float_t>(amp, 0, 1)) / driver::pi * 510));
}

/**
 * @brief Static (Without modulation)
 */
class Static final : public core::Modulation {
 public:
  /**
   * @param[in] amp amplitude
   */
  explicit Static(const driver::autd3_float_t amp = 1.0) noexcept : Modulation(), _amp(amp) {}

  std::vector<uint8_t> calc() override {
    std::vector buffer(2, to_duty(_amp));
    return buffer;
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
  explicit Sine(const int32_t freq, const driver::autd3_float_t amp = 1.0, const driver::autd3_float_t offset = 0.5) noexcept
      : Modulation(), _freq(freq), _amp(amp), _offset(offset) {}

  std::vector<uint8_t> calc() override {
    const auto fs = static_cast<int32_t>(sampling_frequency());

    const auto f = std::clamp(_freq, 1, fs / 2);

    const auto k = std::gcd(fs, f);

    const size_t n = fs / k;
    const size_t d = f / k;

    return generate_iota(0, n, [this, d, n](const size_t i) {
      return to_duty(_amp / 2 * std::sin(2 * driver::pi * static_cast<driver::autd3_float_t>(d * i) / static_cast<driver::autd3_float_t>(n)) +
                     _offset);
    });
  }

  ~Sine() override = default;
  Sine(const Sine& v) noexcept = delete;
  Sine& operator=(const Sine& obj) = delete;
  Sine(Sine&& obj) = default;
  Sine& operator=(Sine&& obj) = default;

 private:
  int32_t _freq;
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
  explicit SineSquared(const int32_t freq, const driver::autd3_float_t amp = 1.0, const driver::autd3_float_t offset = 0.5) noexcept
      : Modulation(), _freq(freq), _amp(amp), _offset(offset) {}

  std::vector<uint8_t> calc() override {
    const auto fs = static_cast<int32_t>(sampling_frequency());

    const auto f = std::clamp(_freq, 1, fs / 2);

    const auto k = std::gcd(fs, f);

    const size_t n = fs / k;
    const size_t d = f / k;

    return generate_iota(0, n, [this, d, n](const size_t i) {
      return to_duty(std::sqrt(
          _amp / 2 * std::sin(2 * driver::pi * static_cast<driver::autd3_float_t>(d * i) / static_cast<driver::autd3_float_t>(n)) + _offset));
    });
  }

  ~SineSquared() override = default;
  SineSquared(const SineSquared& v) noexcept = delete;
  SineSquared& operator=(const SineSquared& obj) = delete;
  SineSquared(SineSquared&& obj) = default;
  SineSquared& operator=(SineSquared&& obj) = default;

 private:
  int32_t _freq;
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

  std::vector<uint8_t> calc() override {
    const auto fs = sampling_frequency();
    const auto f = (std::min)(_freq, fs / 2);

    const auto t = static_cast<size_t>(std::round(fs / f));

    return generate_iota(0, t, [this, t](const size_t i) {
      return to_duty(_offset + _amp * std::cos(2 * driver::pi * static_cast<driver::autd3_float_t>(i) / static_cast<driver::autd3_float_t>(t)) / 2);
    });
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
  explicit Square(const int32_t freq, const driver::autd3_float_t low = 0.0, const driver::autd3_float_t high = 1.0,
                  const driver::autd3_float_t duty = 0.5)
      : _freq(freq), _low(low), _high(high), _duty(duty) {}

  std::vector<uint8_t> calc() override {
    const auto f_s = static_cast<int32_t>(sampling_frequency());
    const auto f = std::clamp(_freq, 1, f_s / 2);
    const auto k = std::gcd(f_s, f);
    const size_t n = f_s / k;
    const size_t d = f / k;

    const auto high = to_duty(_high);
    const auto low = to_duty(_low);

    std::vector buffer(n, low);

    auto* cursor = buffer.data();
    for (size_t i = 0; i < d; i++) {
      const size_t size = (n + i) / d;
      std::memset(cursor, high, static_cast<size_t>(std::round(static_cast<driver::autd3_float_t>(size) * _duty)));
      cursor += size;
    }
    return buffer;
  }

 private:
  int32_t _freq;
  driver::autd3_float_t _low;
  driver::autd3_float_t _high;
  driver::autd3_float_t _duty;
};

template <typename T>
class Cache final : public core::Modulation {
 public:
  template <typename... Args>
  explicit Cache(Args&&... args) : modulation(std::forward<Args>(args)...) {}

  std::vector<uint8_t> calc() override {
    if (!_built) {
      _buffer = modulation.calc();
      _freq_div = modulation.sampling_frequency_division();
      _built = true;
    }
    std::vector<uint8_t> buffer;
    buffer.reserve(_buffer.size());
    std::copy(_buffer.begin(), _buffer.end(), std::back_inserter(buffer));
    return buffer;
  }

  std::vector<uint8_t> recalc() {
    _built = false;
    return calc();
  }

  /**
   * \brief modulation data
   */
  [[nodiscard]] const std::vector<uint8_t>& buffer() const noexcept { return _buffer; }

  /**
   * @brief [Advanced] modulation data
   * @details Call Modulation::build before using this function to initialize buffer data.
   */
  std::vector<uint8_t>& buffer() noexcept { return _buffer; }

  [[nodiscard]] std::vector<uint8_t>::const_iterator begin() const noexcept { return _buffer.begin(); }
  [[nodiscard]] std::vector<uint8_t>::const_iterator end() const noexcept { return _buffer.end(); }
  [[nodiscard]] std::vector<uint8_t>::iterator begin() noexcept { return _buffer.begin(); }
  [[nodiscard]] std::vector<uint8_t>::iterator end() noexcept { return _buffer.end(); }
  [[nodiscard]] const uint8_t& operator[](const size_t i) const { return _buffer[i]; }
  [[nodiscard]] uint8_t& operator[](const size_t i) { return _buffer[i]; }

  T modulation;

 private:
  bool _built{false};
  std::vector<uint8_t> _buffer;
};

}  // namespace autd3::modulation
