// File: info.hpp
// Project: operation
// Created Date: 06/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/cpu/datagram.hpp"

namespace autd3::driver {

struct CPUVersionMajor final {
  static void pack(TxDatagram& tx) {
    tx.header().msg_id = MSG_RD_CPU_VERSION_MAJOR;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_CPU_VERSION_MAJOR);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }
};

struct FPGAVersionMajor final {
  static void pack(TxDatagram& tx) {
    tx.header().msg_id = MSG_RD_FPGA_VERSION_MAJOR;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_FPGA_VERSION_MAJOR);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }
};

struct CPUVersionMinor final {
  static void pack(TxDatagram& tx) {
    tx.header().msg_id = MSG_RD_CPU_VERSION_MINOR;
    tx.num_bodies = 0;
  }
};

struct FPGAVersionMinor final {
  static void pack(TxDatagram& tx) {
    tx.header().msg_id = MSG_RD_FPGA_VERSION_MINOR;
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
