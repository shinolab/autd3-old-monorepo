// File: twincat.cpp
// Project: examples
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3/link/twincat.hpp"

#include "autd3.hpp"
#include "runner.hpp"

int main() try {
  autd3::Geometry geometry;
  geometry.add_device(autd3::core::Vector3::Zero(), autd3::core::Vector3::Zero());

  auto link = autd3::link::TwinCAT(2).build();

  autd3::Controller autd(std::move(link), std::move(geometry));

  return run(std::move(autd));
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
