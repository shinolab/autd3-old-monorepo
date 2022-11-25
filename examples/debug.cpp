// File: debug.cpp
// Project: examples
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/debug.hpp"

#include "autd3.hpp"
#include "runner.hpp"

int main() try {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()));

  if (auto link = autd3::link::Debug().build(); !autd.open(std::move(link))) {
    std::cerr << "Failed to open controller." << std::endl;
    return -1;
  }

  return run(autd);
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
