// File: info.hpp
// Project: operation
// Created Date: 06/12/2022
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

struct CPUVersion final {
  static void pack(TxDatagram& tx) {
    tx.header().msg_id = MSG_RD_CPU_VERSION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_CPU_VERSION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }
};

struct FPGAVersion final {
  static void pack(TxDatagram& tx) {
    tx.header().msg_id = MSG_RD_FPGA_VERSION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_FPGA_VERSION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }
};

struct FPGAFunctions final {
  static void pack(TxDatagram& tx) {
    tx.header().msg_id = MSG_RD_FPGA_FUNCTION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_FPGA_FUNCTION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }
};
}  // namespace autd3::driver
