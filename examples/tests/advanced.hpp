// File: advanced.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3.hpp"

class BurstModulation final : public autd3::Modulation {
 public:
  bool calc() override {
    this->_props.buffer.resize(_buf_size, 0);
    this->_props.buffer.at(_buf_size - 1) = 0xFF;
    return true;
  }

  explicit BurstModulation(const size_t buf_size = 4000, const uint16_t freq_div = 40960) noexcept : _buf_size(buf_size) {
    _props.freq_div = freq_div;
  }

 private:
  size_t _buf_size;
};

class UniformGain final : public autd3::Gain {
 public:
  UniformGain() = default;

  void calc(const autd3::Geometry& geometry) override {
    std::for_each(geometry.begin(), geometry.end(), [this](const auto& dev) {
      std::for_each(dev.begin(), dev.end(), [this](const auto& trans) {
        this->_drives[trans.id()].amp = 1.0;
        this->_drives[trans.id()].phase = 0.0;
      });
    });
  }
};

inline void advanced_test(autd3::Controller& autd) {
  auto config = autd3::SilencerConfig::none();

  autd.geometry()[0][0].mod_delay() = 0;
  autd.geometry()[0][17].mod_delay() = 1;
  autd << autd3::mod_delay_config;

  UniformGain g;
  BurstModulation m;

  autd << config << m, g;
}
