// File: gain.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Gain : public Body {
 public:
  Gain() = default;
  Gain(const Gain& obj) = default;
  Gain& operator=(const Gain& obj) = default;
  Gain(Gain&& obj) = default;
  Gain& operator=(Gain&& obj) = default;
  ~Gain() override = default;

  [[nodiscard]] native_methods::DatagramBodyPtr ptr(const Geometry& geometry) const override { return AUTDGainIntoDatagram(gain_ptr(geometry)); }

  [[nodiscard]] virtual native_methods::GainPtr gain_ptr(const Geometry&) const = 0;
};

}  // namespace autd3::internal
