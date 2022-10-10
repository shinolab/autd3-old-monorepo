// File: c_api.cpp
// Project: link_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <filesystem>
#include <fstream>
#include <string>

#include "./simulator.h"
#include "autd3/extra/simulator/simulator.hpp"

void AUTDExtraSimulator(const char* settings_path) {
  const auto setting_file = std::string(settings_path);

  autd3::extra::simulator::Settings settings;

  if (std::filesystem::exists(setting_file)) {
    std::ifstream i(setting_file);
    nlohmann::json j;
    i >> j;
    settings = j.get<autd3::extra::simulator::Settings>();
  }

  autd3::extra::simulator::Simulator().settings(&settings).run();

  nlohmann::json j = settings;
  std::ofstream o(setting_file);
  o << std::setw(4) << j << std::endl;
}
