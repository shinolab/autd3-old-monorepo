// File: sync.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

template <typename T>
struct Sync final : Operation {
  void pack(TxDatagram& tx) override {
    static_assert(is_mode_v<T>, "Template type parameter must be Mode.");

    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.set(CPUControlFlags::ConfigSync);
    tx.num_bodies = tx.num_devices();

    assert(cycles.size() == tx.bodies_size());
    std::memcpy(tx.bodies_raw_ptr(), cycles.data(), tx.bodies_size());

    _sent = true;
  }

  void init() override {
    _sent = false;
    cycles.clear();
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

  size_t size{};
  std::vector<uint16_t> cycles{};

 private:
  bool _sent{false};
};

template <>
struct Sync<Legacy> final : Operation {
  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.set(CPUControlFlags::ConfigSync);
    tx.num_bodies = tx.num_devices();

    std::generate_n(tx.bodies_raw_ptr(), tx.bodies_size(), [] { return 4096; });
  }

  void init() override { _sent = false; }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
