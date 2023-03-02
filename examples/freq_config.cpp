// File: freq_config.cpp
// Project: examples
// Created Date: 31/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3.hpp"
#include "autd3/link/debug.hpp"
#include "runner.hpp"
#include "util.hpp"

int main() try {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)  // mm/s
                      .build();

  // Here we use link::Debug for example, but you can use any other link.
  auto link = autd3::link::Debug().build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  autd << autd3::advanced_mode;
  std::for_each(autd.geometry().begin(), autd.geometry().end(), [](auto& tr) {
    tr.set_frequency(70e3);  // actual frequency is 163.84MHz/2341 ~ 69987 Hz
  });

  autd << autd3::clear << autd3::synchronize;  // You must configure the frequencies of all transducers before synchronization.

  autd3::SilencerConfig silencer;

  autd3::modulation::Sine m(150);
  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(center);
  autd << silencer << m, g;

  std::cout << "press any key to finish..." << std::endl;
  std::cin.ignore();

  autd.close();

  return 0;
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
