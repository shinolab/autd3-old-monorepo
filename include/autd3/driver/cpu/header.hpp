// File: header.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/driver/fpga/defined.hpp"
#include "defined.hpp"

namespace autd3::driver {

struct SyncHeader {
  uint16_t ecat_sync_cycle_ticks = 0;
  uint16_t pad = 0;
  uint8_t data[120]{};
};

struct ModHead {
  uint32_t freq_div;
  uint8_t data[MOD_HEAD_DATA_SIZE];
};

struct ModBody {
  uint8_t data[MOD_BODY_DATA_SIZE];
};

struct SilencerHeader {
  uint16_t cycle = 0;
  uint16_t step = 0;
  uint8_t data[120]{};
};

struct GlobalHeader {
  uint8_t msg_id;
  FPGAControlFlags fpga_flag;
  CPUControlFlags cpu_flag;
  uint8_t size;
  uint8_t data[124];

  GlobalHeader() noexcept : msg_id(0), fpga_flag(FPGAControlFlags::NONE), cpu_flag(CPUControlFlags::NONE), size(0), data() {}

  void clear() {
    msg_id = 0;
    fpga_flag = FPGAControlFlags::NONE;
    cpu_flag = CPUControlFlags::NONE;
    size = 0;
  }

  [[nodiscard]] const ModHead& mod_head() const { return *reinterpret_cast<ModHead const*>(data); }
  ModHead& mod_head() { return *reinterpret_cast<ModHead*>(data); }

  [[nodiscard]] const ModBody& mod_body() const { return *reinterpret_cast<ModBody const*>(data); }
  ModBody& mod_body() { return *reinterpret_cast<ModBody*>(data); }

  [[nodiscard]] const SyncHeader& sync_header() const { return *reinterpret_cast<SyncHeader const*>(data); }
  SyncHeader& sync_header() { return *reinterpret_cast<SyncHeader*>(data); }

  [[nodiscard]] const SilencerHeader& silencer_header() const { return *reinterpret_cast<SilencerHeader const*>(data); }
  SilencerHeader& silencer_header() { return *reinterpret_cast<SilencerHeader*>(data); }
};
}  // namespace autd3::driver
