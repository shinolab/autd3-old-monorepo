// File: point_stm.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <autd3.hpp>

template <typename T>
void point_stm(autd3::Controller<T>& autd) {
  const auto config = autd3::SilencerConfig::none();
  autd.config_silencer(config);

  autd3::modulation::Static m;

  autd3::PointSTM<T> stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  std::vector<size_t> points(points_num);
  std::iota(points.begin(), points.end(), 0);
  std::transform(points.begin(), points.end(), std::back_inserter(stm), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    return autd3::Point(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
  });

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd.send(m, stm);
}
