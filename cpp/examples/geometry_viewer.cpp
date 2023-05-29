// File: geometry_viewer.cpp
// Project: examples
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/geometry_viewer.hpp"

#include "autd3.hpp"
#include "util.hpp"

int main() try {
  const auto geometry =
      autd3::Geometry::Builder()
          .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))                                           // bottom
          .add_device(autd3::AUTD3(autd3::Vector3(0, 0, autd3::AUTD3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi / 2.0, 0)))  // left
          .add_device(
              autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, autd3::AUTD3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi, 0)))  // top
          .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3(0, -autd3::pi / 2.0, 0)))            // right
          .build();

  autd3::extra::GeometryViewer().window_size(800, 600).vsync(true).run(geometry);

  return 0;
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
