// File: group.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

inline void group_test(autd3::Controller& autd) {
  autd3::SilencerConfig silencer;

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 center = autd.geometry().center_of(0) + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g1(center);

  const autd3::Vector3 apex = autd.geometry().center_of(1);
  autd3::gain::BesselBeam g2(apex, autd3::Vector3::UnitZ(), 13.0 / 180.0 * autd3::pi);

  autd3::gain::Grouped g;
  g.add({ 0, 1 }, g1);
  g.add(1, g2);

  autd << silencer << m, g;
}
