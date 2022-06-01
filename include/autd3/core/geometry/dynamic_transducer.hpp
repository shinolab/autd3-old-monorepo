// File: dynamic_transducer.hpp
// Project: geometry
// Created Date: 24/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <utility>
#include <vector>

#include "autd3/core/utils.hpp"
#include "autd3/driver/cpu/operation.hpp"
#include "autd3/driver/fpga/defined.hpp"
#include "normal_phase_transducer.hpp"
#include "transducer.hpp"

namespace autd3::core {

enum class TransducerMode : uint8_t { Legacy = 0, Normal = 1, NormalPhase = 2 };

/**
 * @brief DriveData for DynamicTransducer
 */
template <typename T>
struct DynamicDriveData final : DriveData<T> {
  void init(const size_t size) override {
    legacy_drives.resize(size, driver::LegacyDrive{0x00, 0x00});
    duties.resize(size, driver::Duty{0x0000});
    phases.resize(size, driver::Phase{0x0000});
  }
  void set_drive(const T& tr, const double phase, const double amp) override {
    switch (T::mode()) {
      case TransducerMode::Legacy:
        legacy_drives.at(tr.id()).set(amp, phase);
        break;
      case TransducerMode::Normal:
        duties.at(tr.id()).set(amp, tr.cycle());
        phases.at(tr.id()).set(phase, tr.cycle());
        break;
      case TransducerMode::NormalPhase:
        phases.at(tr.id()).set(phase, tr.cycle());
        break;
      default:
        legacy_drives.at(tr.id()).set(amp, phase);
        break;
    }
  }

  void copy_from(size_t idx, const typename T::D& src) override {
    const auto* s = src.legacy_drives.data() + idx * driver::NUM_TRANS_IN_UNIT;
    auto* d = legacy_drives.data() + idx * driver::NUM_TRANS_IN_UNIT;
    std::memcpy(d, s, sizeof(driver::LegacyDrive) * driver::NUM_TRANS_IN_UNIT);

    auto ds = src.duties.data() + idx * driver::NUM_TRANS_IN_UNIT;
    const auto dd = duties.data() + idx * driver::NUM_TRANS_IN_UNIT;
    std::memcpy(dd, ds, sizeof(driver::Duty) * driver::NUM_TRANS_IN_UNIT);
    auto ps = src.phases.data() + idx * driver::NUM_TRANS_IN_UNIT;
    const auto pd = phases.data() + idx * driver::NUM_TRANS_IN_UNIT;
    std::memcpy(pd, ps, sizeof(driver::Phase) * driver::NUM_TRANS_IN_UNIT);
  }

  std::vector<driver::LegacyDrive> legacy_drives{};
  std::vector<driver::Duty> duties{};
  std::vector<driver::Phase> phases{};
};

/**
 * \brief Transduce with variable frequency
 */
struct DynamicTransducer final : Transducer<DynamicDriveData<DynamicTransducer>> {
  DynamicTransducer(const size_t id, Vector3 pos, Vector3 x_direction, Vector3 y_direction, Vector3 z_direction) noexcept
      : Transducer(id, std::move(pos), std::move(x_direction), std::move(y_direction), std::move(z_direction)), _cycle(4096) {}
  ~DynamicTransducer() override = default;
  DynamicTransducer(const DynamicTransducer& v) noexcept = default;
  DynamicTransducer& operator=(const DynamicTransducer& obj) = default;
  DynamicTransducer(DynamicTransducer&& obj) = default;
  DynamicTransducer& operator=(DynamicTransducer&& obj) = default;

  [[nodiscard]] uint16_t cycle() const noexcept override { return _cycle; }
  [[nodiscard]] double frequency() const noexcept override { return static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(_cycle); }
  [[nodiscard]] double wavelength(const double sound_speed) const noexcept override { return sound_speed * 1e3 / frequency(); }
  [[nodiscard]] double wavenumber(const double sound_speed) const noexcept override { return 2.0 * driver::pi * frequency() / (sound_speed * 1e3); }

  static void pack_header(driver::TxDatagram& tx) noexcept {
    switch (_mode) {
      case TransducerMode::Legacy:
        normal_legacy_header(tx);
        break;
      case TransducerMode::Normal:
      case TransducerMode::NormalPhase:
        normal_header(tx);
        break;
      default:
        normal_legacy_header(tx);
        break;
    }
  }

  static void pack_body(bool& phase_sent, bool& duty_sent, const D& drives, driver::TxDatagram& tx) noexcept {
    switch (_mode) {
      case TransducerMode::Legacy:
        normal_legacy_body(drives.legacy_drives.data(), tx);
        phase_sent = true;
        duty_sent = true;
        break;

      case TransducerMode::Normal:
        if (!phase_sent) {
          normal_phase_body(drives.phases.data(), tx);
          phase_sent = true;
        } else {
          normal_duty_body(drives.duties.data(), tx);
          duty_sent = true;
        }
        break;
      case TransducerMode::NormalPhase:
        normal_phase_body(drives.phases.data(), tx);
        phase_sent = true;
        duty_sent = true;
        break;
      default:
        normal_legacy_body(drives.legacy_drives.data(), tx);
        phase_sent = true;
        duty_sent = true;
        break;
    }
  }

  void set_cycle(const uint16_t cycle) noexcept {
    switch (_mode) {
      case TransducerMode::Normal:
      case TransducerMode::NormalPhase:
        _cycle = cycle;
        break;
      case TransducerMode::Legacy:
      default:
        break;
    }
  }

  void set_frequency(const double freq) noexcept {
    const auto cycle = static_cast<uint16_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / freq));
    set_cycle(cycle);
  }

  static TransducerMode& mode() { return _mode; }

 private:
  inline static TransducerMode _mode = TransducerMode::Legacy;
  uint16_t _cycle;
};

/**
 * @brief Amplitude configuration for DynamicTransducer.
 */
template <>
struct Amplitudes<DynamicTransducer> final : DatagramBody<DynamicTransducer> {
  explicit Amplitudes(const Geometry<DynamicTransducer>& geometry, const double amp = 1.0) : _sent(false) {
    _duties.resize(geometry.num_transducers());
    for (const auto& dev : geometry)
      for (const auto& tr : dev) _duties.at(tr.id()).set(amp, tr.cycle());
  }
  ~Amplitudes() override = default;
  Amplitudes(const Amplitudes& v) = default;
  Amplitudes& operator=(const Amplitudes& obj) = default;
  Amplitudes(Amplitudes&& obj) = default;
  Amplitudes& operator=(Amplitudes&& obj) = default;

  /**
   * @brief Getter function for the duty data of all transducers
   */
  std::vector<driver::Duty>& duties() { return _duties; }

  void init() override { _sent = false; }

  void pack(const Geometry<DynamicTransducer>&, driver::TxDatagram& tx) override {
    normal_header(tx);
    if (is_finished()) return;
    _sent = true;
    normal_duty_body(_duties.data(), tx);
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _sent; }

 private:
  bool _sent;
  std::vector<driver::Duty> _duties;
};

}  // namespace autd3::core
