// File: autd_simulator.cpp
// Project: simulator
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <filesystem>
#include <fstream>
#include <iostream>

#include "autd3/extra/simulator.hpp"

int main() try {
  const std::string setting_file = "settings.json";

  autd3::extra::SimulatorSettings settings;

  if (std::filesystem::exists(setting_file)) {
    std::ifstream i(setting_file);
    nlohmann::json j;
    i >> j;
    settings = j.get<autd3::extra::SimulatorSettings>();
  } else {
    settings.slice_pos_x = 86.6252f;
    settings.slice_pos_y = 66.7133f;
    settings.slice_pos_z = 150.0f;
    settings.slice_rot_x = 90.0f;
    settings.camera_pos_x = settings.slice_pos_x;
    settings.camera_pos_y = settings.slice_pos_y - 600.0f;
    settings.camera_pos_z = settings.slice_pos_z;
    settings.camera_rot_x = 90.0f;
    settings.port = 50632;
    settings.ip = "127.0.0.1";
  }

  autd3::extra::Simulator().settings(&settings).run();

  nlohmann::json j = settings;
  std::ofstream o(setting_file);
  o << std::setw(4) << j << std::endl;

  return 0;
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
