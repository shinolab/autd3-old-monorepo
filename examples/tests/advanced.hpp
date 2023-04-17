// File: advanced.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3.hpp"

class BurstModulation final : public autd3::Modulation {
 public:
  std::vector<autd3::autd3_float_t> calc() override {
    std::vector<autd3::autd3_float_t> buffer(_buf_size, 0);
    buffer[_buf_size - 1] = 1.0;
    return buffer;
  }

  explicit BurstModulation(const size_t buf_size = 4000, const uint16_t freq_div = 40960) noexcept : Modulation(freq_div), _buf_size(buf_size) {}

 private:
  size_t _buf_size;
};

class UniformGain final : public autd3::Gain {
 public:
  UniformGain() = default;

  std::vector<autd3::driver::Drive> calc(const autd3::Geometry& geometry) override {
    return transform(geometry, [this](const auto&) { return autd3::Drive{0.0, 1.0}; });
  }
};

inline void advanced_test(autd3::Controller& autd) {
  auto config = autd3::SilencerConfig::none();
  autd.send(config);

  autd.geometry()[0].mod_delay = 0;
  autd.geometry()[17].mod_delay = 1;
  autd.send(autd3::ModDelayConfig());

  UniformGain g;
  BurstModulation m;

  autd.send(m, g);
}
