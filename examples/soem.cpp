// File: soem.cpp
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 31/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/soem.hpp"

#include <iostream>

#include "autd3.hpp"
#include "runner.hpp"
#include "util.hpp"

int main() try {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)  // mm/s
                      .build();

  auto link = autd3::link::SOEM()
                  .on_lost([](const std::string& msg) {
                    std::cerr << "Link is lost\n";
                    std::cerr << msg;
#ifdef __APPLE__
                    // mac does not have quick_exit??
                    exit(-1);
#else
                    std::quick_exit(-1);
#endif
                  })
                  .high_precision(true)
                  .build();

  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  autd.set_ack_check_timeout(std::chrono::milliseconds(20));

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
