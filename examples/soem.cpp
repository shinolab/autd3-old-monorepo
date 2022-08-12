// File: soem.cpp
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/soem.hpp"

#include <iostream>

#include "autd3.hpp"
#include "runner.hpp"

int main() try {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());

  auto link = autd3::link::SOEM(autd.geometry().num_devices())
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
  autd.open(std::move(link));

  autd.check_trials = 50;

  return run(std::move(autd));
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
