// File: null.hpp
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

#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct NullHeader final : Operation {
  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSync);
    tx.header().size = 0;
  }

  void init() override {}

  bool is_finished() const override { return true; }
};

struct NullBody final : Operation {
  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.num_bodies = 0;
  }

  void init() override {}
  bool is_finished() const override { return true; }
};

}  // namespace autd3::driver
