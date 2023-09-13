// File: advanced.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <unordered_map>
#include <vector>

#include "autd3.hpp"

class BurstModulation final : public autd3::Modulation {
 public:
  std::vector<double> calc() const override {
    std::vector<double> buffer(_buf_size, 0);
    buffer[_buf_size - 1] = 1.0;
    return buffer;
  }

  explicit BurstModulation(const size_t buf_size = 4000, const uint32_t freq_div = 5120) noexcept
      : autd3::Modulation(freq_div), _buf_size(buf_size) {}

 private:
  size_t _buf_size;
};

class MyUniformGain final : public autd3::Gain {
 public:
  MyUniformGain() = default;

  [[nodiscard]] std::unordered_map<size_t, std::vector<autd3::Drive>> calc(const autd3::Geometry& geometry) const override {
    return autd3::Gain::transform(geometry, [this](const autd3::Device&, const autd3::Transducer&) { return autd3::Drive{0.0, 1.0}; });
  }
};

inline void advanced_test(autd3::Controller& autd) {
  auto config = autd3::Silencer::disable();
  autd.send(config);

  autd.geometry()[0][0].set_mod_delay(0);
  autd.geometry()[0][17].set_mod_delay(1);
  autd.send(autd3::ConfigureModDelay());

  MyUniformGain g;
  BurstModulation m;

  autd.send(m, g);
}
