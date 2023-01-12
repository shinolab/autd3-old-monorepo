// File: silencer.hpp
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

struct ConfigSilencer final {
  void pack(TxDatagram& tx) {
    if (_sent) return;

    if (cycle < SILENCER_CYCLE_MIN)
      throw std::runtime_error("Silencer cycle is out of range. Minimum is " + std::to_string(SILENCER_CYCLE_MIN) + " but you use " +
                               std::to_string(cycle));

    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSync);
    tx.header().cpu_flag.set(CPUControlFlags::ConfigSilencer);

    tx.header().silencer().cycle = cycle;
    tx.header().silencer().step = step;
    _sent = true;
  }

  void init() { _sent = false; }
  [[nodiscard]] bool is_finished() const { return _sent; }

  uint16_t cycle{};
  uint16_t step{};

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
