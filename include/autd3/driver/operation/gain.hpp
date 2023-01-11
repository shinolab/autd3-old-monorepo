// File: gain.hpp
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
#include "operation.hpp"

namespace autd3::driver {

struct GainProps {
  std::vector<Drive> drives{};
};

struct GainBase {
  virtual ~GainBase() = default;
  virtual void init() = 0;
  virtual void pack(TxDatagram& tx) = 0;
  [[nodiscard]] virtual bool is_finished() const = 0;
  GainBase() noexcept = default;
  GainBase(const GainBase& v) noexcept = default;
  GainBase& operator=(const GainBase& obj) = default;
  GainBase(GainBase&& obj) = default;
  GainBase& operator=(GainBase&& obj) = default;
};

template <typename T>
struct Gain;

template <>
struct Gain<Legacy> final : GainBase {
  explicit Gain(GainProps& props) : _props(props) {}

  void init() override {
    _sent = false;
    _props.drives.clear();
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

    assert(_props.drives.size() == tx.bodies_size());
    std::transform(_props.drives.begin(), _props.drives.end(), reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr()), [](const auto& d) { return d; });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  GainProps& _props;
  bool _sent{false};
};

template <>
struct Gain<Normal> final : GainBase {
  explicit Gain(GainProps& props) : _props(props) {}

  void init() override {
    _phase_sent = false;
    _duty_sent = false;
    _props.drives.clear();
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

  std::vector<uint16_t> cycles{};

 private:
  bool _phase_sent{false};
  bool _duty_sent{false};

  GainProps& _props;

  void pack_duty(TxDatagram& tx) const {
    tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(_props.drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    std::transform(_props.drives.begin(), _props.drives.end(), cycles.begin(), reinterpret_cast<Duty*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return Duty(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  void pack_phase(TxDatagram& tx) const {
    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(_props.drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    std::transform(_props.drives.begin(), _props.drives.end(), cycles.begin(), reinterpret_cast<Phase*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return Phase(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }
};

template <>
struct Gain<NormalPhase> final : GainBase {
  explicit Gain(GainProps& props) : _props(props) {}

  void init() override {
    _sent = false;
    _props.drives.clear();
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

    assert(_props.drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    std::transform(_props.drives.begin(), _props.drives.end(), cycles.begin(), reinterpret_cast<Phase*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return Phase(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

  std::vector<uint16_t> cycles{};

 private:
  GainProps& _props;
  bool _sent{false};
};

struct GainDuty final {
  void init() {
    _sent = false;
    drives.clear();
    cycles.clear();
  }

  void pack(TxDatagram& tx) {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);
    tx.num_bodies = 0;

    if (is_finished()) return;
    _sent = true;

    tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

    tx.num_bodies = tx.num_devices();

    assert(drives.size() == tx.bodies_size());
    assert(cycles.size() == tx.bodies_size());
    std::transform(drives.begin(), drives.end(), cycles.begin(), reinterpret_cast<Duty*>(tx.bodies_raw_ptr()),
                   [](const auto& d, const auto cycle) { return Duty(d, cycle); });

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
  }

  [[nodiscard]] bool is_finished() const { return _sent; }

  std::vector<Drive> drives{};
  std::vector<uint16_t> cycles{};

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
