// File: lpf_wrapper.hpp
// Project: base
// Created Date: 11/04/2023
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"
#include "autd3/modulation/lpf.hpp"

class LPF4CAPI : autd3::Modulation {
 public:
  explicit LPF4CAPI(autd3::core::Modulation* modulation) : Modulation(8192), _lpf(modulation) {}
  std::vector<autd3::driver::float_t> calc() override { return _lpf.calc(); }

 private:
  autd3::modulation::LPF<Modulation*> _lpf;
};
