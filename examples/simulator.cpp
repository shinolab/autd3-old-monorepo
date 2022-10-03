// File: simulator_example.cpp
// Project: examples
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/simulator/simulator.hpp"

#include "autd3.hpp"
#include "autd3/link/simulator.hpp"
#include "runner.hpp"

int main() try {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0.0, 0.0), autd3::Vector3::Zero());

  // autd.geometry().mode() = std::make_unique<autd3::NormalMode>();
  // for (auto& dev : autd.geometry())
  //   for (auto& tr : dev) tr.set_frequency(70e3);

  auto link = autd3::link::Simulator().port(50632).build();
  autd.open(std::move(link));

  autd3::extra::simulator::Simulator simulator =
      autd3::extra::simulator::Simulator().vsync(true).shader(AUTD3_SIMULATOR_SHADER_PATH).texture(AUTD3_SIMULATOR_TEXTURE_PATH);

  bool sim_run = false;
  auto th = std::thread([&sim_run, &simulator]() { simulator.start(&sim_run); });

  run(std::move(autd));

  sim_run = false;
  if (th.joinable()) th.join();

  return 0;
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
