// File: geometry_viewer_example.cpp
// Project: examples
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <iostream>

#include "autd3.hpp"
#include "autd3/extra/geometry_viewer/geometry_viewer.hpp"

int main() try {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());                                                // bottom
  autd.geometry().add_device(autd3::Vector3(0, 0, autd3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi / 2.0, 0));              // left
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, autd3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi, 0));  // top
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0, 0), autd3::Vector3(0, -autd3::pi / 2.0, 0));             // right

  autd3::extra::geometry_viewer::GeometryViewer().window_size(800, 600).vsync(true).view(autd.geometry());

  return 0;

} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
