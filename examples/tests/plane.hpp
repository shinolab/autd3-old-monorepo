// File: plane.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

template <typename T>
void plane_test(autd3::ControllerX<T>& autd) {
  autd3::SilencerConfig config;
  autd.send(config);

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 direction = autd3::Vector3::UnitZ();
  autd3::gain::PlaneWave<T> g(direction);

  autd.send(m, g);
}
