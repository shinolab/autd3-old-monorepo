// File: focus.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/07/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

inline void focus_test(autd3::Controller& autd) {
  autd3::SilencerConfig config;
  autd.send(config);

  autd3::modulation::Sine m(150);  // 150Hz AM

  // const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  // autd3::gain::Focus g(center);

  // autd.send(m, g);

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 5000;
  constexpr auto radius = 30.0;
  std::vector<size_t> points(points_num);
  std::iota(points.begin(), points.end(), 0);
  std::for_each(points.begin(), points.end(), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(200);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
    const auto r = autd.send(m, g);
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
    std::cout << "\x1b[K" << std::boolalpha << r << "\r";
  });
}
