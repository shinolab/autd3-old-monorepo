// File: sync.hpp
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

#include <vector>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct SyncBase {
  virtual ~SyncBase() = default;
  virtual void init() = 0;
  virtual void pack(TxDatagram& tx) = 0;
  [[nodiscard]] bool is_finished() const { return _sent; }

 protected:
  bool _sent{false};
};

template <typename T>
struct Sync final : SyncBase {
  void pack(TxDatagram& tx) override {
    static_assert(is_mode_v<T>, "Template type parameter must be Mode.");

    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.set(CPUControlFlags::ConfigSync);
    tx.num_bodies = tx.num_devices();

    assert(cycles.size() == tx.bodies_size());
    std::copy_n(cycles.begin(), tx.bodies_size(), tx.bodies_raw_ptr());

    _sent = true;
  }

  void init() override {
    _sent = false;
    cycles.clear();
  }

  std::vector<uint16_t> cycles{};
};

template <>
struct Sync<Legacy> final : SyncBase {
  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.set(CPUControlFlags::ConfigSync);
    tx.num_bodies = tx.num_devices();

    std::generate_n(tx.bodies_raw_ptr(), tx.bodies_size(), [] { return 4096; });

    _sent = true;
  }

  void init() override { _sent = false; }
};

}  // namespace autd3::driver
