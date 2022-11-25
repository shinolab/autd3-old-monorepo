// File: group.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

inline void group_test(autd3::Controller& autd) {
  autd3::SilencerConfig silencer;

  autd3::modulation::Sine m(150);  // 150Hz AM

  const auto second_dev_tr_idx = autd.geometry().device_map()[0];
  autd3::Vector3 first_center = autd3::Vector3::Zero();
  autd3::Vector3 second_center = autd3::Vector3::Zero();
  for (size_t i = 0; i < second_dev_tr_idx; i++) first_center += autd.geometry()[i].position();
  first_center /= static_cast<double>(second_dev_tr_idx);
  for (size_t i = second_dev_tr_idx; i < autd.geometry().num_transducers(); i++) second_center += autd.geometry()[i].position();
  second_center /= static_cast<double>(autd.geometry().num_transducers() - second_dev_tr_idx);

  const autd3::Vector3 center = first_center + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g1(center);

  const autd3::Vector3 apex = second_center;
  autd3::gain::BesselBeam g2(apex, autd3::Vector3::UnitZ(), 13.0 / 180.0 * autd3::pi);

  autd3::gain::Grouped g(autd.geometry());
  g.add(0, g1);
  g.add(1, g2);

  autd << silencer << m, g;
}
