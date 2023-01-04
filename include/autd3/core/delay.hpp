// File: delay.hpp
// Project: core
// Created Date: 01/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <vector>

#include "datagram.hpp"
#include "geometry.hpp"

namespace autd3::core {

/**
 * @brief ModDelayConfig is a DatagramBody to configure modulation delay
 */
struct ModDelayConfig final : DatagramBody {
  ModDelayConfig() : _sent(false) {}
  ~ModDelayConfig() override = default;
  ModDelayConfig(const ModDelayConfig& v) = default;
  ModDelayConfig& operator=(const ModDelayConfig& obj) = default;
  ModDelayConfig(ModDelayConfig&& obj) = default;
  ModDelayConfig& operator=(ModDelayConfig&& obj) = default;

  bool init() override {
    _sent = false;
    return true;
  }

  bool pack(Mode, const Geometry& geometry, driver::TxDatagram& tx) override {
    if (!driver::NullBody().pack(tx)) return false;
    if (is_finished()) return true;

    std::vector<uint16_t> delays;
    delays.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(delays), [](const Transducer& tr) { return tr.mod_delay(); });

    _sent = true;
    return driver::ModDelay().delays(delays).pack(tx);
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _sent; }

 private:
  bool _sent;
};

}  // namespace autd3::core
