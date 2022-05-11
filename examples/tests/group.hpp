// File: group.hpp
// Project: examples
// Created Date: 23/05/2021
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include "autd3.hpp"

template <typename T>
void group_test(autd3::Controller<T>& autd) {
  const auto config = autd3::SilencerConfig();
  autd.config_silencer(config);

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 center = autd.geometry()[0].center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g1(center);

  const autd3::Vector3 apex = autd.geometry()[1].center();
  autd3::gain::BesselBeam g2(apex, autd3::Vector3::UnitZ(), 13.0 / 180.0 * autd3::pi);

  autd3::gain::Grouped g(autd.geometry());
  g.add(0, g1);
  g.add(1, g2);

  autd.send(m, g);
}
