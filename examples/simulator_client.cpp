// File: simulator_client.cpp
// Project: examples
// Created Date: 07/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3.hpp"
#include "autd3/link/simulator.hpp"
#include "runner.hpp"

// Run example_simulator_server before running this example

int main() try {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()));
  autd.geometry().add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0.0, 0.0), autd3::Vector3::Zero()));

  // autd << autd3::normal_mode;
  // for (auto& tr : autd.geometry()) tr.set_frequency(70e3);

  if (auto link = autd3::link::Simulator().build(); !autd.open(std::move(link))) {
    std::cerr << "Failed to open controller." << std::endl;
    return -1;
  }

  return run(autd);
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
