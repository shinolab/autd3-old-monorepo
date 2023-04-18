// File: soft_stm.hpp
// Project: tests
// Created Date: 06/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>

inline void soft_stm(autd3::Controller& autd) {
  auto silencer = autd3::SilencerConfig::none();
  autd.send(silencer);

  autd3::modulation::Static m;
  autd.send(m);

  autd3::SoftwareSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  for (size_t i = 0; i < points_num; i++) {
    constexpr auto radius = 30.0;
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add(autd3::gain::Focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0)));
  }

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";

  auto handle = stm.start(autd);

  std::cout << "press any key to stop software stm..." << std::endl;
  std::cin.ignore();

  handle.finish();
}
