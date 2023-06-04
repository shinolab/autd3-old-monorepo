// File: geometry_viewer.cpp
// Project: examples
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/geometry_viewer.hpp"

#include "autd3.hpp"
#include "autd3/link/debug.hpp"
#include "util.hpp"

int main() try {
  const auto autd = autd3::Controller::builder()
                        .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))                                           // bottom
                        .add_device(autd3::AUTD3(autd3::Vector3(0, 0, autd3::AUTD3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi / 2.0, 0)))  // left
                        .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, autd3::AUTD3::DEVICE_WIDTH),
                                                 autd3::Vector3(0, autd3::pi, 0)))                                                           // top
                        .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3(0, -autd3::pi / 2.0, 0)))  // right
                        .open_with(autd3::link::Debug());

  return autd3::extra::GeometryViewer().window_size(800, 600).vsync(true).run(autd.geometry());
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
