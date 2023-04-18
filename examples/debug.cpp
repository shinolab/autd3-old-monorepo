// File: debug.cpp
// Project: examples
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/debug.hpp"

#include "autd3.hpp"
#include "runner.hpp"
#include "util.hpp"

int main() try {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)  // mm/s
                      .build();

  auto link = autd3::link::Debug().build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
