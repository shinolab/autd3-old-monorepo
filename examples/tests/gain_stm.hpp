// File: gain_stm.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>

inline void gain_stm(autd3::Controller& autd) {
  auto silencer = autd3::SilencerConfig::none();
  autd.send(silencer, std::chrono::milliseconds(20));

  autd3::modulation::Static m;

  autd3::GainSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 50;
  for (size_t i = 0; i < points_num; i++) {
    constexpr auto radius = 30.0;
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0));
    stm.add(g);
  }

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd.send(m, stm, std::chrono::milliseconds(20));
}
