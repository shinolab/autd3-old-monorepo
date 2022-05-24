// File: advanced.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
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

template <typename T = autd3::LegacyTransducer, std::enable_if_t<std::is_base_of_v<autd3::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class UniformGain final : public autd3::Gain<T> {
 public:
  UniformGain() = default;

  void calc(const autd3::Geometry<T>& geometry) override {
    std::for_each(geometry.begin(), geometry.end(), [this](const auto& dev) {
      std::for_each(dev.begin(), dev.end(), [this](const auto& trans) { this->_props.drives.set_drive(trans, 0.0, 1.0); });
    });
  }
};

template <typename T>
void advanced_test(autd3::Controller<T>& autd) {
  auto config = autd3::SilencerConfig::none();
  autd.send(config);

  UniformGain<T> g;
  BurstModulation m;

  autd.send(m, g);
}
