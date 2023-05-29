// File: primitive.hpp
// Project: modulation
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

class Static : public internal::Modulation {
 public:
  Static(const double amp = 1.0) : internal::Modulation(internal::native_methods::AUTDModulationStatic(amp)) {}
};

class Sine : public internal::Modulation {
 public:
  Sine(const int32_t freq, const double amp = 1.0, const double offset = 0.5)
      : internal::Modulation(internal::native_methods::AUTDModulationSine(freq, amp, offset)) {}
};

class SineSquared : public internal::Modulation {
 public:
  SineSquared(const int32_t freq, const double amp = 1.0, const double offset = 0.5)
      : internal::Modulation(internal::native_methods::AUTDModulationSineSquared(freq, amp, offset)) {}
};

class SineLegacy : public internal::Modulation {
 public:
  SineLegacy(const double freq, const double amp = 1.0, const double offset = 0.5)
      : internal::Modulation(internal::native_methods::AUTDModulationSineLegacy(freq, amp, offset)) {}
};

class Square : public internal::Modulation {
 public:
  Square(const int32_t freq, const double low = 0.0, const double high = 1.0, const double duty = 0.5)
      : internal::Modulation(internal::native_methods::AUTDModulationSquare(freq, low, high, duty)) {}
};

class Modulation : public internal::Modulation {
 public:
  Modulation(const uint16_t freq_div = 5120) : internal::Modulation(nullptr), _freq_div(freq_div) {}

  virtual std::vector<double> calc() = 0;

  void* ptr() override {
    const auto buffer = calc();
    const auto size = static_cast<uint64_t>(buffer.size());
    _ptr = internal::native_methods::AUTDModulationCustom(buffer.data(), size, _freq_div);
    return _ptr;
  }

 private:
  uint16_t _freq_div;
};

}  // namespace autd3::modulation
