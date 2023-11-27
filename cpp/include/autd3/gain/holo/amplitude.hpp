// File: amplitude.hpp
// Project: holo
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

namespace autd3::gain::holo {
class Amplitude;

class UnitPascal {
  friend Amplitude operator*(double l, const UnitPascal&);
};
class UnitSPL {
  friend Amplitude operator*(double l, const UnitSPL&);
};

constexpr UnitPascal Pascal = UnitPascal{};
constexpr UnitSPL dB = UnitSPL{};

class Amplitude {
 public:
  friend class UnitPascal;
  friend class UnitSPL;

  [[nodiscard]] double as_pascal() const { return _value; }
  [[nodiscard]] double as_spl() const { return internal::native_methods::AUTDGainHoloPascalToSPL(_value); }

  friend Amplitude operator*(const double l, const UnitPascal&) { return Amplitude(l); }
  friend Amplitude operator*(const double l, const UnitSPL&) { return Amplitude(internal::native_methods::AUTDGainHoloSPLToPascal(l)); }

 private:
  explicit Amplitude(const double value) : _value(value) {}

  double _value;
};

}  // namespace autd3::gain::holo