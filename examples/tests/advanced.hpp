// File: advanced.hpp
// Project: examples
// Created Date: 19/05/2021
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3.hpp"

class BurstModulation final : public autd3::core::Modulation {
 public:
  void calc() override {
    this->_props.buffer.resize(_buf_size, 0);
    this->_props.buffer[_buf_size - 1] = 0xFF;
  }

  explicit BurstModulation(const size_t buf_size = 4000, const uint16_t freq_div = 40960) : _buf_size(buf_size) { _props.freq_div = freq_div; }

 private:
  size_t _buf_size;
};

template <typename T = autd3::core::LegacyTransducer,
          std::enable_if_t<std::is_base_of_v<autd3::core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class UniformGain final : public autd3::core::Gain<T> {
 public:
  UniformGain() = default;

  void calc(const autd3::Geometry<T>& geometry) override {
    for (const auto& dev : geometry)
      for (const auto& trans : dev) this->_props.drives.set_drive(trans, 0.0, 1.0);
  }
};

template <typename T>
void advanced_test(autd3::Controller<T>& autd) {
  const auto config = autd3::SilencerConfig::none();
  autd.config_silencer(config);

  UniformGain g;
  BurstModulation m;
  autd.send(m, g);
}
