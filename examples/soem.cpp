// File: soem.cpp
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/07/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/soem.hpp"

#include <iostream>

#include "autd3.hpp"
#include "runner.hpp"

std::string get_adapter_name() {
  size_t i = 0;
  const auto adapters = autd3::link::SOEM::enumerate_adapters();
  for (auto&& [desc, name] : adapters) std::cout << "[" << i++ << "]: " << desc << ", " << name << std::endl;

  std::cout << "Choose number: ";
  std::string in;
  getline(std::cin, in);
  std::stringstream s(in);
  if (const auto empty = in == "\n"; !(s >> i) || i >= adapters.size() || empty) return "";

  return adapters[i].name;
}

int main() try {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());

  const auto ifname = get_adapter_name();
  auto link = autd3::link::SOEM(ifname, autd.geometry().num_devices())
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
                  .cycle_ticks(2)
                  .high_precision(true)
                  .build();
  autd.open(std::move(link));

  autd.check_trials = 50;

  return run(std::move(autd));
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
