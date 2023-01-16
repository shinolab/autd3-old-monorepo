// File: advanced.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <vector>

#include "autd3.hpp"

class BurstModulation final : public autd3::Modulation {
 public:
  std::vector<uint8_t> calc() override {
    std::vector<uint8_t> buffer(_buf_size, 0);
    buffer[_buf_size - 1] = 0xFF;
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
    std::vector<autd3::driver::Drive> drives;
    drives.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(drives), [this](const auto&) { return autd3::driver::Drive{0.0, 1.0}; });
    return drives;
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
