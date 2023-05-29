// File: simulator_server.cpp
// Project: examples
// Created Date: 07/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include <filesystem>

#include "autd3/extra/simulator.hpp"
#include "util.hpp"

int main([[maybe_unused]] int argc, char* argv[]) try {

	const auto path = std::filesystem::path("settings.json");

  auto simulator = autd3::extra::Simulator().settings_path(path);
  simulator.run();

  simulator.save_settings(path);

  return 0;
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
