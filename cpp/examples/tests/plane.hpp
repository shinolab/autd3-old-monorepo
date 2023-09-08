// File: plane.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

inline void plane_test(autd3::Controller& autd) {
  autd3::Silencer silencer;
  autd.send(silencer);

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 direction = autd3::Vector3::UnitZ();
  autd3::gain::Plane g(direction);

  autd.send(m, g);
}
