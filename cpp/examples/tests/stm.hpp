// File: stm.hpp
// Project: tests
// Created Date: 08/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>
#include <ranges>

using namespace std::ranges::views;

template <typename L>
inline void focus_stm(autd3::Controller<L>& autd) {
  auto silencer = autd3::Silencer::disable();
  autd.send_async(silencer).get();

  autd3::modulation::Static m;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  auto stm = autd3::FocusSTM(1).add_foci_from_iter(iota(0) | take(points_num) | transform([&](auto i) {
                                                     const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
                                                     autd3::Vector3 p =
                                                         center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0);
                                                     return p;
                                                   }));

  std::cout << "Actual frequency is " << stm.frequency() << " Hz\n";

  autd.send_async(m, stm).get();
}

template <typename L>
inline void gain_stm(autd3::Controller<L>& autd) {
  auto silencer = autd3::Silencer::disable();
  autd.send_async(silencer).get();

  autd3::modulation::Static m;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 50;
  constexpr auto radius = 30.0;
  auto stm = autd3::GainSTM(1).add_gains_from_iter(iota(0) | take(points_num) | transform([&](auto i) {
                                                     const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
                                                     return autd3::gain::Focus(center +
                                                                               autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
                                                   }));

  std::cout << "Actual frequency is " << stm.frequency() << " Hz\n";
  autd.send_async(m, stm).get();
}
