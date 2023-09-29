// File: freq_config.cpp
// Project: examples
// Created Date: 31/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include <ranges>

#include "autd3.hpp"
#include "autd3/link/debug.hpp"
#include "util.hpp"

int main() try {
  // Here we use link::Debug for example, but you can use any other link.
  auto autd = autd3::Controller::builder()
                  .advanced()
                  .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                  .open_with(autd3::link::Debug());

  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_frequency(70e3);  // actual frequency is 163.84MHz/2341 ~ 69987 Hz

  autd.send(autd3::Synchronize());  // You must synchronize after configuring the frequencies.

  return 0;
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
