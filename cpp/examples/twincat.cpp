// File: twincat.cpp
// Project: examples
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/twincat.hpp"

#include "autd3.hpp"
#include "runner.hpp"
#include "util.hpp"

int main() try {
  const auto geometry = autd3::Geometry::Builder().add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero())).build();

  const auto link = autd3::link::TwinCAT().build();

  auto autd = autd3::Controller::open(geometry, link);

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
