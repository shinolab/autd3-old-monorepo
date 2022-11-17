// File: holo.hpp
// Project: tests
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <iostream>
#include <memory>
#include <string>
#include <tuple>
#include <vector>

#include "autd3.hpp"
#include "autd3/gain/holo.hpp"

inline void holo_test(autd3::Controller& autd) {
  autd3::SilencerConfig silencer;

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  std::cout << "Select Optimization Method (default is GSPAT)" << std::endl;

  auto backend = autd3::gain::holo::EigenBackend::create();

  std::vector<std::tuple<std::string, std::shared_ptr<autd3::gain::holo::Holo>>> opts;
  opts.emplace_back(std::make_tuple("SDP", std::make_shared<autd3::gain::holo::SDP>(backend)));
  opts.emplace_back(std::make_tuple("EVD", std::make_shared<autd3::gain::holo::EVD>(backend)));
  opts.emplace_back(std::make_tuple("GS", std::make_shared<autd3::gain::holo::GS>(backend)));
  opts.emplace_back(std::make_tuple("GSPAT", std::make_shared<autd3::gain::holo::GSPAT>(backend)));
  opts.emplace_back(std::make_tuple("Naive", std::make_shared<autd3::gain::holo::Naive>(backend)));
  opts.emplace_back(std::make_tuple("LM", std::make_shared<autd3::gain::holo::LM>(backend)));
  opts.emplace_back(std::make_tuple("Greedy", std::make_shared<autd3::gain::holo::Greedy>(backend)));
  opts.emplace_back(std::make_tuple("LSSGreedy", std::make_shared<autd3::gain::holo::LSSGreedy>(backend)));
  opts.emplace_back(std::make_tuple("APO", std::make_shared<autd3::gain::holo::APO>(backend)));

  size_t i = 0;
  for (const auto& [name, _opt] : opts) std::cout << "[" << i++ << "]: " << name << std::endl;

  std::string in;
  size_t idx;
  getline(std::cin, in);
  std::stringstream s(in);
  if (const auto empty = in == "\n"; !(s >> idx) || idx >= opts.size() || empty) idx = 3;

  auto& [_, g] = opts[idx];
  g->add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0);
  g->add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0);
  g->add_focus(center + autd3::Vector3(0.0, 30.0, 0.0), 1.0);
  g->add_focus(center - autd3::Vector3(0.0, 30.0, 0.0), 1.0);

  g->constraint = std::make_unique<autd3::gain::holo::Uniform>(1.0);

  autd << silencer << m, *g;
}
