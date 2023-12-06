// File: flag.hpp
// Project: tests
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iterator>
#include <thread>
#include <vector>

#include "autd3.hpp"

template <typename L>
inline void flag_test(autd3::Controller<L>& autd) {
  std::cout << "press any key to run fan..." << std::endl;
  std::cin.ignore();

  autd.send_async(autd3::ConfigureForceFan([](const auto&) { return true; }), autd3::ConfigureReadsFPGAInfo([](const auto&) { return true; })).get();

  bool fin = false;
  auto check_states_thread = std::thread([&] {
    const std::vector prompts = {'-', '/', '|', '\\'};
    size_t prompts_idx = 0;
    while (!fin) {
      const auto states = autd.fpga_info_async().get();
      std::cout << prompts[prompts_idx++ / 1000 % prompts.size()] << " FPGA Status...\n";
      std::copy(states.begin(), states.end(), std::ostream_iterator<autd3::FPGAInfo>(std::cout, "\n"));
      std::cout << "\033[" << states.size() + 1 << "A";
    }
  });

  std::cout << "press any key stop checking FPGA status..." << std::endl;
  std::cin.ignore();

  fin = true;
  if (check_states_thread.joinable()) check_states_thread.join();

  autd.send_async(autd3::ConfigureForceFan([](const auto&) { return false; }), autd3::ConfigureReadsFPGAInfo([](const auto&) { return false; }))
      .get();
}
