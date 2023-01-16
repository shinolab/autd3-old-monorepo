// File: sync.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 17/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <utility>
#include <vector>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

template <typename T>
struct Sync final : Operation {
  explicit Sync(std::vector<uint16_t> cycles) : _cycles(std::move(cycles)) {}

  void pack(TxDatagram& tx) override {
    static_assert(is_mode_v<T>, "Template type parameter must be Mode.");

    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.set(CPUControlFlags::ConfigSync);
    tx.num_bodies = tx.num_devices();

    assert(_cycles.size() == tx.bodies_size());
    std::copy_n(_cycles.begin(), tx.bodies_size(), tx.bodies_raw_ptr());

    _sent = true;
  }

  void init() override { _sent = false; }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  bool _sent{false};
  std::vector<uint16_t> _cycles{};
};

template <>
struct Sync<Legacy> final : Operation {
  Sync() = default;

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.set(CPUControlFlags::ConfigSync);
    tx.num_bodies = tx.num_devices();

    std::generate_n(tx.bodies_raw_ptr(), tx.bodies_size(), [] { return 4096; });

    _sent = true;
  }

  void init() override { _sent = false; }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
