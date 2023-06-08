// File: simulator_server.cpp
// Project: examples
// Created Date: 07/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include <filesystem>

#include "autd3/extra/simulator.hpp"
#include "util.hpp"

int main() try {
  const auto path = std::filesystem::path("settings.json");

  const auto simulator = autd3::extra::Simulator().settings_path(path);
  const auto res = simulator.run();

  simulator.save_settings(path);

  return res;
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
