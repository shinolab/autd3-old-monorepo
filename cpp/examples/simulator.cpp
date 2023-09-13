// File: simulator.cpp
// Project: examples
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 14/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/simulator.hpp"

#include "autd3.hpp"
#include "runner.hpp"
#include "util.hpp"

// Run AUTD Simulator before running this example

int main() try {
  auto autd = autd3::Controller::builder()
                  .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                  .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0.0, 0.0), autd3::Vector3::Zero()))
                  .open_with(autd3::link::Simulator(8080).with_timeout(std::chrono::milliseconds(200)));

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
