// File: advanced.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3.hpp"

class BurstModulation final : public autd3::Modulation {
 public:
  std::vector<autd3::Amp> calc() override {
    std::vector buffer(_buf_size, autd3::Amp(0));
    buffer[_buf_size - 1] = autd3::Amp(1.0);
    return buffer;
  }

  explicit BurstModulation(const size_t buf_size = 4000, const uint16_t freq_div = 40960) noexcept : _buf_size(buf_size) { _freq_div = freq_div; }

 private:
  size_t _buf_size;
};

class UniformGain final : public autd3::Gain {
 public:
  UniformGain() = default;

  std::vector<autd3::driver::Drive> calc(const autd3::Geometry& geometry) override {
    return autd3::Gain::transform(geometry, [this](const auto&) { return autd3::Drive{autd3::Phase(0), autd3::Amp(1)}; });
  }
};

inline void advanced_test(autd3::Controller& autd) {
  auto config = autd3::SilencerConfig::none();

  autd.geometry()[0].mod_delay() = 0;
  autd.geometry()[17].mod_delay() = 1;
  autd << autd3::mod_delay_config;

  UniformGain g;
  BurstModulation m;

  autd << config << m, g;
}
