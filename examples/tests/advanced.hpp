// File: advanced.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3.hpp"

class BurstModulation final : public autd3::Modulation {
 public:
  void calc() override {
    this->_props.buffer.resize(_buf_size, 0);
    this->_props.buffer.at(_buf_size - 1) = 0xFF;
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
    std::ranges::for_each(geometry, [this](const auto& dev) {
      std::ranges::for_each(dev, [this](const auto& trans) {
        this->_drives[trans.id()].amp = 1.0;
        this->_drives[trans.id()].phase = 0.0;
      });
    });
  }
};

inline void advanced_test(autd3::Controller& autd) {
  auto config = autd3::SilencerConfig::none();
  autd.send(config);

  // autd.geometry()[0][0].mod_delay() = 0;
  // autd.geometry()[0][17].mod_delay() = 1;
  // autd.send(autd3::ModDelayConfig());

  UniformGain g;
  BurstModulation m;

  autd.send(m, g);
}
