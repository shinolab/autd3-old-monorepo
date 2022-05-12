// File: primitive.hpp
// Project: gain
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "autd3/core/modulation.hpp"
#include "autd3/driver/hardware.hpp"

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

  void calc() override {
    this->_props.buffer.resize(2, 0);
    for (size_t i = 0; i < 2; i++) {
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp(_amp, 0.0, 1.0)) / driver::pi * 510.0));
      this->_props.buffer.at(i) = duty;
    }
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

  void calc() override {
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

}  // namespace autd3::modulation
