// File: soem.cpp
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/soem.hpp"

#include <iostream>

#include "autd3.hpp"
#include "runner.hpp"
#include "util.hpp"

[[noreturn]] void on_lost(const char* msg) {
  std::cerr << "Link is lost\n";
  std::cerr << msg;
#ifdef __APPLE__
  // mac does not have quick_exit??
  exit(-1);
#else
  std::quick_exit(-1);
#endif
}

int main() try {
  auto autd = autd3::Controller::builder()
                  .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                  .open_with(autd3::link::SOEM().with_on_lost(&on_lost));

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
