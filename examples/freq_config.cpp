// File: freq_config.cpp
// Project: examples
// Created Date: 31/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3.hpp"
#include "autd3/link/debug.hpp"
#include "runner.hpp"

int main() try {
  spdlog::set_level(spdlog::level::debug);

  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());

  // Here we use link::Debug for example, but you can use any other link.
  auto link = autd3::link::Debug().build();
  autd.open(std::move(link));

  autd.mode() = autd3::NormalMode::create();
  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_frequency(70e3);  // actual frequency is 163.84MHz/2341 ~ 69987 Hz

  autd.send(autd3::Clear{});
  autd.send(autd3::Synchronize{});  // You must configure the frequencies of all transducers before synchronization.

  autd3::SilencerConfig config;
  autd.send(config);

  autd3::modulation::Sine m(150);
  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(center);
  autd.send(m, g);

  std::cout << "press any key to finish..." << std::endl;
  std::cin.ignore();

  autd.close();

  return run(autd);
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
