// File: gain.hpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

template <typename T>
struct Gain;

template <>
struct Gain<Legacy> final : Operation {
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
    auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr());
    for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

  std::vector<Drive> drives{};

 private:
  bool _sent{false};
};

template <>
struct Gain<Normal> final : Operation {
  void init() override {
    _phase_sent = false;
    _duty_sent = false;
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

    if (!_phase_sent) {
      _phase_sent = true;
      pack_phase(tx);
      return;
    }

    _duty_sent = true;
    pack_duty(tx);
  }

  [[nodiscard]] bool is_finished() const override { return _phase_sent && _duty_sent; }

  std::vector<Drive> drives{};
  std::vector<uint16_t> cycles{};

 private:
  bool _phase_sent{false};
  bool _duty_sent{false};

  void pack_duty(TxDatagram& tx) const {
    tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    auto* p = reinterpret_cast<Duty*>(tx.bodies_raw_ptr());
    for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i], cycles[i]);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  void pack_phase(TxDatagram& tx) const {
    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    auto* p = reinterpret_cast<Phase*>(tx.bodies_raw_ptr());
    for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i], cycles[i]);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }
};

template <>
struct Gain<NormalPhase> final : Operation {
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
    auto* p = reinterpret_cast<Phase*>(tx.bodies_raw_ptr());
    for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i], cycles[i]);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

  std::vector<Drive> drives{};
  std::vector<uint16_t> cycles{};

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
