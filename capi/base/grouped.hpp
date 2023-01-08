// File: grouped.hpp
// Project: base
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <vector>

#include "autd3.hpp"

/**
 * @brief Grouped for capi
 */
class Grouped4CAPI final : public autd3::core::Gain {
 public:
  void add(const size_t device_id, autd3::core::Gain* gain) { _gains.insert_or_assign(device_id, gain); }

  void calc(const autd3::core::Geometry& geometry) override {
    std::for_each(_gains.begin(), _gains.end(), [this, geometry](const auto& g) {
      const auto& [device_id, gain] = g;
      gain->init(_mode, geometry);
      const auto start = device_id == 0 ? 0 : geometry.device_map()[device_id - 1];
      std::memcpy(&_op->drives[start], &gain->drives()[start], sizeof(autd3::driver::Drive) * geometry.device_map()[device_id]);
    });
  }

  Grouped4CAPI() : autd3::core::Gain() {}
  ~Grouped4CAPI() override = default;
  Grouped4CAPI(const Grouped4CAPI& v) noexcept = delete;
  Grouped4CAPI& operator=(const Grouped4CAPI& obj) = delete;
  Grouped4CAPI(Grouped4CAPI&& obj) = delete;
  Grouped4CAPI& operator=(Grouped4CAPI&& obj) = delete;

 private:
  std::unordered_map<size_t, autd3::core::Gain*> _gains{};
};
