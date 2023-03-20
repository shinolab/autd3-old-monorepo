// File: gain.hpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 14/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cassert>
#include <utility>
#include <vector>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

template <typename T>
struct Gain;

template <>
struct Gain<Legacy> final : Operation {
  explicit Gain(std::vector<Drive> drives) : _drives(std::move(drives)) {}

  void init() override { _sent = false; }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);

    tx.header().fpga_flag.set(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);

    tx.num_bodies = 0;

    if (is_finished()) return;

    tx.num_bodies = tx.num_devices();

    assert(_drives.size() == tx.num_transducers());
    std::transform(_drives.begin(), _drives.end(), reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr()), [](const auto& d) { return d; });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    _sent = true;
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  std::vector<Drive> _drives{};
  bool _sent{false};
};

template <>
struct Gain<Advanced> final : Operation {
  explicit Gain(std::vector<Drive> drives, const std::vector<uint16_t>& cycles) : _drives(std::move(drives)), _cycles(cycles) {}

  void init() override {
    _phase_sent = false;
    _duty_sent = false;
  }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);
    tx.num_bodies = 0;

    if (is_finished()) return;

    if (!_phase_sent) {
      pack_phase(tx);
      _phase_sent = true;
      return;
    }

    pack_duty(tx);
    _duty_sent = true;
  }

  [[nodiscard]] bool is_finished() const override { return _phase_sent && _duty_sent; }

 private:
  bool _phase_sent{false};
  bool _duty_sent{false};
  std::vector<Drive> _drives{};
  const std::vector<uint16_t>& _cycles;

  void pack_duty(TxDatagram& tx) const {
    tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(_drives.size() == tx.num_transducers());
    assert(_cycles.size() == tx.num_transducers());
    std::transform(_drives.begin(), _drives.end(), _cycles.begin(), reinterpret_cast<AdvancedDriveDuty*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return AdvancedDriveDuty(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  void pack_phase(TxDatagram& tx) const {
    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(_drives.size() == tx.num_transducers());
    assert(_cycles.size() == tx.num_transducers());
    std::transform(_drives.begin(), _drives.end(), _cycles.begin(), reinterpret_cast<AdvancedDrivePhase*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return AdvancedDrivePhase(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }
};

template <>
struct Gain<AdvancedPhase> final : Operation {
  explicit Gain(std::vector<Drive> drives, const std::vector<uint16_t>& cycles) : _drives(std::move(drives)), _cycles(cycles) {}

  void init() override { _sent = false; }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);
    tx.num_bodies = 0;

    if (is_finished()) return;

    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(_drives.size() == tx.num_transducers());
    assert(_cycles.size() == tx.num_transducers());
    std::transform(_drives.begin(), _drives.end(), _cycles.begin(), reinterpret_cast<AdvancedDrivePhase*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return AdvancedDrivePhase(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    _sent = true;
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  std::vector<Drive> _drives{};
  const std::vector<uint16_t>& _cycles;
  bool _sent{false};
};

struct Amplitude final : Operation {
  explicit Amplitude(std::vector<Drive> drives, const std::vector<uint16_t>& cycles) : _drives(std::move(drives)), _cycles(cycles) {}

  void init() override { _sent = false; }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);
    tx.num_bodies = 0;

    if (is_finished()) return;

    tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(_drives.size() == tx.num_transducers());
    assert(_cycles.size() == tx.num_transducers());
    std::transform(_drives.begin(), _drives.end(), _cycles.begin(), reinterpret_cast<AdvancedDriveDuty*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return AdvancedDriveDuty(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    _sent = true;
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  std::vector<Drive> _drives{};
  const std::vector<uint16_t>& _cycles;
  bool _sent{false};
};

}  // namespace autd3::driver
