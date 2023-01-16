// File: silencer.hpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 16/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct ConfigSilencer final : Operation {
  ConfigSilencer(const uint16_t cycle, const uint16_t step) : _cycle(cycle), _step(step) {}

  void pack(TxDatagram& tx) override {
    if (_sent) return;

    if (_cycle < SILENCER_CYCLE_MIN)
      throw std::runtime_error("Silencer _cycle is out of range. Minimum is " + std::to_string(SILENCER_CYCLE_MIN) + " but you use " +
                               std::to_string(_cycle));

    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSync);
    tx.header().cpu_flag.set(CPUControlFlags::ConfigSilencer);

    tx.header().silencer().cycle = _cycle;
    tx.header().silencer().step = _step;
    _sent = true;
  }

  void init() override { _sent = false; }
  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  bool _sent{false};
  uint16_t _cycle{};
  uint16_t _step{};
};

}  // namespace autd3::driver
