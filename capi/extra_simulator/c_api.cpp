﻿// File: c_api.cpp
// Project: link_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <filesystem>
#include <fstream>
#include <string>

#include "../../src/spdlog.hpp"
#include "./simulator.h"
#include "autd3/extra/simulator.hpp"

bool AUTDExtraSimulator(const char* settings_path, const bool vsync, const int32_t gpu_idx) {
  try {
    const auto setting_file = std::string(settings_path);

    autd3::extra::SimulatorSettings settings;
    settings.vsync = vsync;
    settings.gpu_idx = gpu_idx;

    if (std::filesystem::exists(setting_file)) {
      std::ifstream i(setting_file);
      nlohmann::json j;
      i >> j;
      settings = j.get<autd3::extra::SimulatorSettings>();
    }

    autd3::extra::Simulator().settings(settings).run();

    nlohmann::json j = settings;
    std::ofstream o(setting_file);
    o << std::setw(4) << j << std::endl;
    return true;
  } catch (std::exception& ex) {
    spdlog::error(ex.what());

    return false;
  }
}
