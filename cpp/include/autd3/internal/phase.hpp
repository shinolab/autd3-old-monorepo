// File: emit_intensity.hpp
// Project: internal
// Created Date: 12/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Phase;

class UnitPhaseRad {
  friend Phase operator*(double l, const UnitPhaseRad&);
};

constexpr UnitPhaseRad rad = UnitPhaseRad{};

class Phase final {
 public:
  explicit Phase(const uint8_t value) : _value(value) {}

  [[nodiscard]] static Phase from_rad(const double value) { return Phase(native_methods::AUTDPhaseFromRad(value)); }

  [[nodiscard]] double radian() { return native_methods::AUTDPhaseToRad(_value); }

  [[nodiscard]] uint8_t value() const noexcept { return _value; }

  auto operator<=>(const Phase&) const = default;

 private:
  uint8_t _value;
};

}  // namespace autd3::internal
