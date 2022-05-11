// File: gain_stm.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <autd3.hpp>

template <typename T>
void gain_stm(autd3::Controller<T>& autd) {
  const auto config = autd3::SilencerConfig::none();
  autd.config_silencer(config);

  autd3::modulation::Static m;

  autd3::GainSTM<T> stm(autd.geometry());

  const auto center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr auto point_num = 200;
  for (auto i = 0; i < point_num; i++) {
    constexpr auto radius = 30.0;
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(point_num);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0));
    stm.add(g);
  }

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd.send(m, stm);
}
