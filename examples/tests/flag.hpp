// File: flag.hpp
// Project: tests
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "autd3.hpp"

template <typename T>
void flag_test(autd3::Controller<T>& autd) {
  autd.reads_fpga_info = true;

  std::cout << "press any key to run fan..." << std::endl;
  std::cin.ignore();

  autd.force_fan = true;
  autd.update_flag();

  while (true) {
    const auto states = autd.read_fpga_info();
    for (size_t i = 0; i < autd.geometry().num_devices(); i++)
      std::cout << "[" << i << "]: fan = " << std::boolalpha << states.at(i).is_fan_running() << std::endl;
  }

  autd.force_fan = false;
  autd.update_flag();
}
