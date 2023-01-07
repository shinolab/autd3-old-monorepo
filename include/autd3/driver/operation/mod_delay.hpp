// File: mod_delay.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cassert>
#include <vector>

#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct ModDelay final : Operation {
  void init() override {
    _sent = false;
    delays.clear();
  }

  void pack(TxDatagram& tx) override {
    if (_sent) return;

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.set(CPUControlFlags::ModDelay);
    tx.num_bodies = tx.num_devices();

    assert(delays.size() == tx.bodies_size());
    std::copy_n(delays.begin(), tx.bodies_size(), tx.bodies_raw_ptr());
    _sent = true;
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

  std::vector<uint16_t> delays{};

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
