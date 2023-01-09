// File: delay.hpp
// Project: core
// Created Date: 01/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <vector>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/mod_delay.hpp"

namespace autd3::core {

/**
 * @brief ModDelayConfig is a DatagramBody to configure modulation delay
 */
struct ModDelayConfig final : DatagramBody {
  ModDelayConfig() = default;
  ~ModDelayConfig() override = default;
  ModDelayConfig(const ModDelayConfig& v) = default;
  ModDelayConfig& operator=(const ModDelayConfig& obj) = default;
  ModDelayConfig(ModDelayConfig&& obj) = default;
  ModDelayConfig& operator=(ModDelayConfig&& obj) = default;

  void init(const Mode, const Geometry& geometry) override {
    _op.init();
    _op.delays.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(_op.delays), [](const Transducer& tr) { return tr.mod_delay(); });
  }

  void pack(driver::TxDatagram& tx) override { _op.pack(tx); }

  [[nodiscard]] bool is_finished() const noexcept override { return _op.is_finished(); }

 private:
  driver::ModDelay _op;
};

}  // namespace autd3::core
