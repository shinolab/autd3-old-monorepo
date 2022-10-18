// File: soft_stm.hpp
// Project: tests
// Created Date: 06/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>
#include <utility>
#include <vector>

inline void soft_stm(autd3::Controller& autd) {
  auto config = autd3::SilencerConfig::none();
  autd.send(config);

  autd3::modulation::Static m;
  autd.send(m);

  autd3::SoftwareSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  std::vector<size_t> points(points_num);
  std::iota(points.begin(), points.end(), 0);
  std::for_each(points.begin(), points.end(), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add(autd3::gain::Focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0)));
  });

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";

  auto handle = stm.start(std::move(autd));

  std::cout << "press any key to stop software stm..." << std::endl;
  std::cin.ignore();

  autd = handle.finish();
}
