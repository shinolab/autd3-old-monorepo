// File: gain.hpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct GainBase : Operation {
  std::vector<Drive> drives{};
};

template <typename T>
struct Gain;

template <>
struct Gain<Legacy> final : GainBase {
  void init() override {
    _sent = false;
    drives.clear();
  }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);

    tx.header().fpga_flag.set(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);

    tx.num_bodies = 0;
    if (_sent) return;
    _sent = true;

    tx.num_bodies = tx.num_devices();

    assert(drives.size() == tx.bodies_size());
    std::transform(drives.begin(), drives.end(), reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr()), [](const auto& d) { return d; });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  bool _sent{false};
};

template <>
struct Gain<Normal> final : GainBase {
  void init() override {
    phase_sent = false;
    duty_sent = false;
    drives.clear();
    cycles.clear();
  }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);
    tx.num_bodies = 0;
    if (is_finished()) return;

    if (!phase_sent) {
      phase_sent = true;
      pack_phase(tx);
      return;
    }

    duty_sent = true;
    pack_duty(tx);
  }

  [[nodiscard]] bool is_finished() const override { return phase_sent && duty_sent; }

  std::vector<uint16_t> cycles{};

  bool phase_sent{false};
  bool duty_sent{false};

 private:
  void pack_duty(TxDatagram& tx) const {
    tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    std::transform(drives.begin(), drives.end(), cycles.begin(), reinterpret_cast<Duty*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return Duty(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  void pack_phase(TxDatagram& tx) const {
    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    std::transform(drives.begin(), drives.end(), cycles.begin(), reinterpret_cast<Phase*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return Phase(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }
};

template <>
struct Gain<NormalPhase> final : GainBase {
  void init() override {
    _sent = false;
    drives.clear();
    cycles.clear();
  }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);
    tx.num_bodies = 0;
    if (is_finished()) return;

    _sent = true;

    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    std::transform(drives.begin(), drives.end(), cycles.begin(), reinterpret_cast<Phase*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return Phase(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

  std::vector<uint16_t> cycles{};

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
