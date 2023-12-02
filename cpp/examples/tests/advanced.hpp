// File: advanced.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
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
  std::vector<autd3::EmitIntensity> calc() const override {
    std::vector<autd3::EmitIntensity> buffer(_buf_size, autd3::EmitIntensity::minimum());
    buffer[_buf_size - 1] = autd3::EmitIntensity::maximum();
    return buffer;
  }

  explicit BurstModulation(const size_t buf_size = 4000,
                           const autd3::SamplingConfiguration config = autd3::SamplingConfiguration::from_frequency(4e3)) noexcept
      : autd3::Modulation(config), _buf_size(buf_size) {}

 private:
  size_t _buf_size;
};

class MyUniformGain final : public autd3::Gain {
 public:
  MyUniformGain() = default;

  [[nodiscard]] std::unordered_map<size_t, std::vector<autd3::Drive>> calc(const autd3::Geometry& geometry) const override {
    return autd3::Gain::transform(geometry, [this](const autd3::Device&, const autd3::Transducer&) {
      return autd3::Drive{autd3::Phase(0), autd3::EmitIntensity::maximum()};
    });
  }
};

template <typename L>
inline void advanced_test(autd3::Controller<L>& autd) {
  auto config = autd3::Silencer::disable();
  autd.send_async(config).get();

  MyUniformGain g;
  BurstModulation m;

  autd.send_async(m, g).get();
}
