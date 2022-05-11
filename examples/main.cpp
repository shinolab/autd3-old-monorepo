// File: main.cpp
// Project: examples
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include <iostream>

#include "autd3.hpp"
#include "autd3/link/emulator.hpp"

int main() {
  auto geometry = autd3::Geometry();

  geometry.add_device(autd3::core::Vector3::Zero(), autd3::core::Vector3::Zero());

  auto link = autd3::link::Emulator(geometry).port(50632).build();

  auto autd = autd3::Controller(std::move(link), std::move(geometry));

  std::cout << "***** Firmware information *****" << std::endl;
  for (const auto& firm_info : autd.firmware_infos()) std::cout << firm_info << std::endl;
  std::cout << "********************************" << std::endl;

  autd.clear();

  autd.synchronize();

  auto m = autd3::modulation::Sine(150);
  auto g = autd3::gain::Focus(autd3::core::Vector3(90., 70., 150.), 1.0);
  //   autd.send(m, g);
  autd.send(autd3::modulation::Sine(150), autd3::gain::Focus(autd3::core::Vector3(90., 70., 150.), 1.0));

  int i;
  std::cin >> i;

  autd.close();

  std::cout << "fin" << std::endl;

  return 0;
}
