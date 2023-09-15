// File: stm.hpp
// Project: tests
// Created Date: 08/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 15/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>

#if __cplusplus >= 202002L
#include <ranges>
using namespace std::ranges::views;
#endif

inline void focus_stm(autd3::Controller& autd) {
  auto silencer = autd3::Silencer::disable();
  autd.send(silencer);

  autd3::modulation::Static m;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
#if __cplusplus >= 202002L
  auto stm = autd3::FocusSTM(1).add_foci_from_iter(iota(0) | take(points_num) | transform([&](auto i) {
                                                     const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
                                                     autd3::Vector3 p =
                                                         center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0);
                                                     return p;
                                                   }));
#else
  autd3::FocusSTM stm(1);
  for (size_t i = 0; i < points_num; i++) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add_focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
  }
#endif

  std::cout << "Actual frequency is " << stm.frequency() << " Hz\n";

  autd.send(m, stm);
}

inline void gain_stm(autd3::Controller& autd) {
  auto silencer = autd3::Silencer::disable();
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
    stm.add_gain(g);
  }
#endif

  std::cout << "Actual frequency is " << stm.frequency() << " Hz\n";
  autd.send(m, stm);
}

inline void software_stm(autd3::Controller& autd) {
  auto silencer = autd3::Silencer::disable();
  autd.send(silencer);

  autd3::modulation::Static m;
  autd.send(m);

  bool fin = false;
  auto th = std::thread([&] {
    std::cout << "press enter to stop software stm..." << std::endl;
    std::cin.ignore();
    fin = true;
  });

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr double freq = 1.0;
  constexpr size_t points_num = 100;
  constexpr auto radius = 30.0;
  autd.software_stm([&fin, radius, points_num, center](autd3::Controller& autd, size_t i, std::chrono::nanoseconds elapsed) {
        if (fin) return false;
        const auto theta = 2.0 * autd3::pi * (i % points_num) / points_num;
        const autd3::Vector3 p = radius * autd3::Vector3(std::cos(theta), std::sin(theta), 0.0);
        try {
          return autd.send(autd3::gain::Focus(center + p));
        } catch (std::exception& e) {
          return false;
        }
      })
      .with_timer_strategy(autd3::TimerStrategy::NativeTimer)
      .start(std::chrono::nanoseconds(static_cast<uint64_t>(1000000000.0 / freq / points_num)));

  if (th.joinable()) th.join();
}
