// File: reads_fpga_info.hpp
// Project: operation
// Created Date: 07/01/2023
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

struct ReadsFPGAInfo final : Operation {
  void pack(TxDatagram& tx) override {
    if (value)
      tx.header().fpga_flag.set(FPGAControlFlags::ReadsFPGAInfo);
    else
      tx.header().fpga_flag.remove(FPGAControlFlags::ReadsFPGAInfo);
  }

  void init() override {}
  [[nodiscard]] bool is_finished() const override { return true; }

  bool value{false};
};

}  // namespace autd3::driver
