// File: flag.hpp
// Project: tests
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iterator>
#include <thread>
#include <vector>

#include "autd3.hpp"

template <typename T>
void flag_test(autd3::ControllerX<T>& autd) {
  autd.reads_fpga_info = true;

  std::cout << "press any key to run fan..." << std::endl;
  std::cin.ignore();

  autd.force_fan = true;
  autd.update_flag();

  bool fin = false;
  auto check_states_thread = std::thread([&] {
    const std::vector prompts = {'-', '/', '|', '\\'};
    size_t prompts_idx = 0;
    while (!fin) {
      const auto states = autd.read_fpga_info();
      std::cout << prompts[prompts_idx] << " FPGA Status: ";
      std::copy(states.begin(), states.end(), std::ostream_iterator<autd3::FPGAInfo>(std::cout, ", "));
      std::cout << "\r";
      prompts_idx = (prompts_idx + 1) % prompts.size();
    }
  });

  std::cout << "press any key stop checking fpga states..." << std::endl;
  std::cin.ignore();

  fin = true;
  if (check_states_thread.joinable()) check_states_thread.join();

  autd.force_fan = false;
  autd.update_flag();
}
