// File: focus_stm.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>

inline void focus_stm(autd3::Controller& autd) {
  auto silencer = autd3::SilencerConfig::none();
  autd.send(silencer);

  autd3::modulation::Static m;

  autd3::FocusSTM stm(1);

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  for (size_t i = 0; i < points_num; i++) {
    constexpr auto radius = 30.0;
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add_focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
  }

  std::cout << "Actual frequency is " << stm.frequency() << " Hz\n";

  autd.send(m, stm);
}
