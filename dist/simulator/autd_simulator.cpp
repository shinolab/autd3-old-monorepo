// File: autd_simulator.cpp
// Project: simulator
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 23/12/2022
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
  }

  if (!autd3::extra::Simulator().settings(settings).run()) {
    std::cerr << "Failed to run simulator." << std::endl;
    return -1;
  }

  nlohmann::json j = settings;
  std::ofstream o(setting_file);
  o << std::setw(4) << j << std::endl;

  return 0;
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
