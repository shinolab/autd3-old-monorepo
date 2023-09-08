// File: holo.hpp
// Project: tests
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iostream>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "autd3.hpp"
#include "autd3/gain/holo.hpp"

inline void holo_test(autd3::Controller& autd) {
  autd3::Silencer silencer;
  autd.send(silencer);

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  std::cout << "Select Optimization Method (default is GSPAT)" << std::endl;

  std::cout << "[0]: SDP" << std::endl;
  std::cout << "[1]: EVP" << std::endl;
  std::cout << "[2]: GS" << std::endl;
  std::cout << "[3]: GSPAT" << std::endl;
  std::cout << "[4]: Naive" << std::endl;
  std::cout << "[5]: LM" << std::endl;
  std::cout << "[6]: Greedy" << std::endl;
  std::cout << "[Others]: GS-PAT" << std::endl;
  std::cout << "Choose number: ";

  std::string in;
  size_t idx;
  getline(std::cin, in);
  std::stringstream s(in);
  if (const auto empty = in == "\n"; !(s >> idx) || idx >= 7 || empty) idx = 3;

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  switch (idx) {
    case 0:
      autd.send(m, autd3::gain::holo::SDP(backend)
                       .add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center + autd3::Vector3(0.0, 30.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(0.0, 30.0, 0.0), 1.0));
      break;
    case 1:
      autd.send(m, autd3::gain::holo::EVP(backend)
                       .add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center + autd3::Vector3(0.0, 30.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(0.0, 30.0, 0.0), 1.0));
      break;
    case 2:
      autd.send(m, autd3::gain::holo::GS(backend)
                       .add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center + autd3::Vector3(0.0, 30.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(0.0, 30.0, 0.0), 1.0));
      break;
    case 3:
      autd.send(m, autd3::gain::holo::GSPAT(backend)
                       .add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center + autd3::Vector3(0.0, 30.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(0.0, 30.0, 0.0), 1.0));
      break;
    case 4:
      autd.send(m, autd3::gain::holo::Naive(backend)
                       .add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center + autd3::Vector3(0.0, 30.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(0.0, 30.0, 0.0), 1.0));
      break;
    case 5:
      autd.send(m, autd3::gain::holo::LM(backend)
                       .add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center + autd3::Vector3(0.0, 30.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(0.0, 30.0, 0.0), 1.0));
      break;
    case 6:
      autd.send(m, autd3::gain::holo::Greedy()
                       .add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0)
                       .add_focus(center + autd3::Vector3(0.0, 30.0, 0.0), 1.0)
                       .add_focus(center - autd3::Vector3(0.0, 30.0, 0.0), 1.0));
      break;
    default:
      break;
  }
}
