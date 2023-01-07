// File: flag.hpp
// Project: tests
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <iterator>
#include <thread>
#include <vector>

#include "autd3.hpp"

inline void flag_test(autd3::Controller& autd) {
  autd.reads_fpga_info() = true;

  std::cout << "press any key to run fan..." << std::endl;
  std::cin.ignore();

  autd.force_fan() = true;
  autd << autd3::update_flag;

  bool fin = false;
  auto check_states_thread = std::thread([&] {
    const std::vector prompts = {'-', '/', '|', '\\'};
    size_t prompts_idx = 0;
    while (!fin) {
      const auto states = autd.fpga_info();
      std::cout << prompts[prompts_idx++ / 1000 % prompts.size()] << " FPGA Status...\n";
      std::copy(states.begin(), states.end(), std::ostream_iterator<autd3::FPGAInfo>(std::cout, "\n"));
      std::cout << "\033[" << states.size() + 1 << "A";
    }
  });

  std::cout << "press any key stop checking FPGA status..." << std::endl;
  std::cin.ignore();

  fin = true;
  if (check_states_thread.joinable()) check_states_thread.join();

  autd.force_fan() = false;
  autd << autd3::update_flag;
}
