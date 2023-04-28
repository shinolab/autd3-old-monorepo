// File: remote_simulator.cpp
// Project: examples
// Created Date: 28/04/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/remote_simulator.hpp"

#include "autd3.hpp"
#include "runner.hpp"
#include "util.hpp"

int main() try {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)  // mm/s
                      .build();

  const std::string ip = "simulator IP";
  constexpr uint16_t port = 0;  // simulator port

  auto link = autd3::link::RemoteSimulator(ip, port).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
