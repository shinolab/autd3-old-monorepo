// File: null.hpp
// Project: operation
// Created Date: 06/01/2023
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

struct NullHeader final {
  void pack(TxDatagram& tx) {
    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSync);
    tx.header().size = 0;
    _sent = true;
  }

  void init() { _sent = false; }

  bool is_finished() const { return _sent; }

 private:
  bool _sent{false};
};

struct NullBody final {
  void pack(TxDatagram& tx) {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.num_bodies = 0;
    _sent = true;
  }

  void init() { _sent = false; }

  bool is_finished() const { return _sent; }

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
