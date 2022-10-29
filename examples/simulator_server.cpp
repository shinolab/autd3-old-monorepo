// File: simulator_server.cpp
// Project: examples
// Created Date: 07/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <filesystem>
#include <iostream>

#include "autd3/extra/simulator.hpp"

int main([[maybe_unused]] int argc, char* argv[]) try {
  autd3::extra::SimulatorSettings settings;
  settings.slice_pos_x = 182.625f;
  settings.slice_pos_y = 66.7133f;
  settings.slice_pos_z = 150.0f;
  settings.slice_rot_x = 90.0f;
  settings.camera_pos_x = settings.slice_pos_x;
  settings.camera_pos_y = settings.slice_pos_y - 600.0f;
  settings.camera_pos_z = settings.slice_pos_z;
  settings.camera_rot_x = 90.0f;
  settings.image_save_path = std::filesystem::path(argv[0]).parent_path().append("image.png").string();

  autd3::extra::Simulator().settings(&settings).run();

  return 0;
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
