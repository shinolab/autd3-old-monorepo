// File: geometry_viewer.cpp
// Project: examples
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/geometry_viewer.hpp"

#include <iostream>

#include "autd3.hpp"

int main() try {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());                                                // bottom
  autd.geometry().add_device(autd3::Vector3(0, 0, autd3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi / 2.0, 0));              // left
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, autd3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi, 0));  // top
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3(0, -autd3::pi / 2.0, 0));             // right

  autd3::extra::GeometryViewer().window_size(800, 600).vsync(true).model(AUTD3_GEOMETRY_VIEWER_MODEL_PATH).view(autd.geometry());

  return 0;
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
