// File: amplitudes.hpp
// Project: internal
// Created Date: 04/06/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Amplitudes final : public Body {
 public:
  Amplitudes() noexcept : Amplitudes(1.0) {}
  explicit Amplitudes(const double amp) noexcept : _amp(amp) {}
  Amplitudes(const Amplitudes& obj) = default;
  Amplitudes& operator=(const Amplitudes& obj) = default;
  Amplitudes(Amplitudes&& obj) = default;
  Amplitudes& operator=(Amplitudes&& obj) = default;
  ~Amplitudes() override = default;

  [[nodiscard]] native_methods::DatagramBodyPtr ptr(const Geometry&) const override { return native_methods::AUTDCreateAmplitudes(_amp); }

 private:
  double _amp;
};

}  // namespace autd3::internal
