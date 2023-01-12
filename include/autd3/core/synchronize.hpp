// File: synchronize.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/sync.hpp"

namespace autd3::core {

/**
 * @brief DatagramBody for synchronization
 */
struct Synchronize final : DatagramBody {
  Synchronize() noexcept = default;

  void init(const Mode mode, const Geometry& geometry) override {
    switch (mode) {
      case Mode::Legacy: {
        if (const auto cycles = geometry.cycles(); std::any_of(cycles.begin(), cycles.end(), [](const auto& cycle) { return cycle != 4096; }))
          throw std::runtime_error("Frequency cannot be changed in Legacy mode.");
        auto op = std::make_unique<driver::Sync<driver::Legacy>>();
        op->init();
        _op = std::move(op);
      } break;
      case Mode::Normal: {
        auto op = std::make_unique<driver::Sync<driver::Normal>>();
        op->init();
        op->cycles = geometry.cycles();
        _op = std::move(op);
      } break;
      case Mode::NormalPhase: {
        auto op = std::make_unique<driver::Sync<driver::NormalPhase>>();
        op->init();
        op->cycles = geometry.cycles();
        _op = std::move(op);
      } break;
    }
  }

  void pack(driver::TxDatagram& tx) override { _op->pack(tx); }

  [[nodiscard]] bool is_finished() const override { return _op->is_finished(); }

 private:
  std::unique_ptr<driver::SyncBase> _op;
};

}  // namespace autd3::core
