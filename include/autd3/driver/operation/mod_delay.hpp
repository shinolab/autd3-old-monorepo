// File: mod_delay.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cassert>
#include <utility>
#include <vector>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct ModDelay final : Operation {
  explicit ModDelay(std::vector<uint16_t> delays) : _delays(std::move(delays)) {}

  void init() override { _sent = false; }

  void pack(TxDatagram& tx) override {
    if (_sent) return;

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.set(CPUControlFlags::ModDelay);
    tx.num_bodies = tx.num_devices();

    assert(_delays.size() == tx.num_transducers());
    std::copy_n(_delays.begin(), tx.num_transducers(), tx.bodies_raw_ptr());
    _sent = true;
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  bool _sent{false};
  std::vector<uint16_t> _delays{};
};

}  // namespace autd3::driver
