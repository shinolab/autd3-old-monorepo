// File: gain_stm.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 23/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>

#if __cplusplus >= 202002L
#include <ranges>
using namespace std::ranges::views;
#endif

inline void gain_stm(autd3::Controller& autd) {
  auto silencer = autd3::SilencerConfig::none();
  autd.send(silencer);

  autd3::modulation::Static m;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 50;
  constexpr auto radius = 30.0;
#if __cplusplus >= 202002L
  auto stm = autd3::GainSTM(1).add_gains_from_iter(iota(0) | take(points_num) | transform([&](auto i) {
                                                     const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
                                                     return autd3::gain::Focus(center +
                                                                               autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
                                                   }));
#else
  autd3::GainSTM stm(1);
  for (size_t i = 0; i < points_num; i++) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0));
    stm = stm.add_gain(g);
  }
#endif

  std::cout << "Actual frequency is " << stm.frequency() << " Hz\n";
  autd.send(m, stm);
}
