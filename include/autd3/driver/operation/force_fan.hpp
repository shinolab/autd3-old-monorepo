// File: force_fan.hpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/cpu/datagram.hpp"

namespace autd3::driver {

struct ForceFan final {
  void pack(TxDatagram& tx) const {
    if (value)
      tx.header().fpga_flag.set(FPGAControlFlags::ForceFan);
    else
      tx.header().fpga_flag.remove(FPGAControlFlags::ForceFan);
  }

  bool value{false};
};

}  // namespace autd3::driver
