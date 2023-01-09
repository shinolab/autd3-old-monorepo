// File: info.hpp
// Project: operation
// Created Date: 06/12/2022
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

struct CPUVersion final : Operation {
  void pack(TxDatagram& tx) override {
    tx.header().msg_id = MSG_RD_CPU_VERSION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_CPU_VERSION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }

  void init() override {}
  [[nodiscard]] bool is_finished() const override { return true; }
};

struct FPGAVersion final : Operation {
  void pack(TxDatagram& tx) override {
    tx.header().msg_id = MSG_RD_FPGA_VERSION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_FPGA_VERSION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }

  void init() override {}
  [[nodiscard]] bool is_finished() const override { return true; }
};

struct FPGAFunctions final : Operation {
  void pack(TxDatagram& tx) override {
    tx.header().msg_id = MSG_RD_FPGA_FUNCTION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_FPGA_FUNCTION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }

  void init() override {}
  [[nodiscard]] bool is_finished() const override { return true; }
};
}  // namespace autd3::driver
