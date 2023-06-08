// File: simulator_client.cpp
// Project: examples
// Created Date: 07/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include "autd3.hpp"
#include "autd3/link/simulator.hpp"
#include "runner.hpp"
#include "util.hpp"

// Run example_simulator_server before running this example

int main() try {
  auto autd = autd3::Controller::builder()
                  .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                  .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0.0, 0.0), autd3::Vector3::Zero()))
                  // .advanced_mode()
                  .open_with(autd3::link::Simulator(8080).with_timeout(std::chrono::milliseconds(200)));

  // std::for_each(autd.geometry().begin(), autd.geometry().end(), [](auto& tr) { tr.set_frequency(70e3); });

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
