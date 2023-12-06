// File: gain.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Gain {
 public:
  Gain() = default;
  Gain(const Gain& obj) = default;
  Gain& operator=(const Gain& obj) = default;
  Gain(Gain&& obj) = default;
  Gain& operator=(Gain&& obj) = default;
  virtual ~Gain() = default;  // LCOV_EXCL_LINE

  [[nodiscard]] native_methods::DatagramPtr ptr(const geometry::Geometry& geometry) const { return AUTDGainIntoDatagram(gain_ptr(geometry)); }

  [[nodiscard]] virtual native_methods::GainPtr gain_ptr(const geometry::Geometry& geometry) const = 0;
};

template <class G>
concept gain = std::derived_from<std::remove_reference_t<G>, Gain>;

}  // namespace autd3::internal
