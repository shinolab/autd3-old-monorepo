// File: synchronize.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/sync.hpp"

namespace autd3::core {

/**
 * @brief DatagramBody for synchronization
 */
template <typename T>
struct Synchronize final : DatagramBody {
  Synchronize() noexcept = default;

  bool init(const Geometry& geometry) override {
    _op.init();
    if constexpr (driver::uses_cycle_v<T>) _op.cycles = geometry.cycles();
    return true;
  }

  void pack(driver::TxDatagram& tx) override { _op.pack(tx); }

  [[nodiscard]] bool is_finished() const override { return true; }

 private:
  driver::Sync<T> _op;
};

}  // namespace autd3::core
