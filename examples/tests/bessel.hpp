// File: bessel.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <autd3.hpp>

template <typename T>
void bessel_test(autd3::Controller<T>& autd) {
  const auto config = autd3::SilencerConfig();
  autd.config_silencer(config);

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 apex = autd.geometry().center();
  autd3::gain::BesselBeam g(apex, autd3::Vector3::UnitZ(), 13.0 / 180.0 * autd3::pi);

  autd.send(m, g);
}
