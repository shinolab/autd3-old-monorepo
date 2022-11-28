// File: custom_device.cpp
// Project: examples
// Created Date: 28/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3.hpp"
#include "autd3/link/simulator.hpp"
#include "runner.hpp"

class ConcentricArray final : autd3::core::Device {
 public:
  ConcentricArray() = default;

  [[nodiscard]] std::vector<autd3::core::Transducer> get_transducers(const size_t start_id) const override {
    std::vector<autd3::core::Transducer> transducers;
    size_t id = start_id;
    transducers.emplace_back(autd3::core::Transducer(id++, autd3::Vector3::Zero(), autd3::Quaternion::Identity()));
    for (size_t layer = 1; layer <= 5; layer++) {
      for (size_t i = 0; i < 6 * layer; i++) {
        const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(6 * layer);
        const autd3::Vector3 pos = static_cast<double>(layer) * 10.0 * autd3::Vector3(std::cos(theta), std::sin(theta), 0);
        transducers.emplace_back(autd3::core::Transducer(id++, pos, autd3::Quaternion::Identity()));
      }
    }
    return transducers;
  }
};

int main() try {
  autd3::Controller autd;

  autd.geometry().add_device(ConcentricArray());

  if (auto link = autd3::link::Simulator().build(); !autd.open(std::move(link))) {
    std::cerr << "Failed to open controller." << std::endl;
    return -1;
  }

  return run(autd);
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
