// File: dynamic_transducer.hpp
// Project: geometry
// Created Date: 24/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
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
#include "transducer.hpp"

namespace autd3::core {

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
    if (T::legacy_mode()) {
      legacy_drives.at(tr.id()).duty = static_cast<uint8_t>(std::round(510.0 * std::asin(std::clamp(amp, 0.0, 1.0)) / driver::pi));
      legacy_drives.at(tr.id()).phase = static_cast<uint8_t>(static_cast<int32_t>(std::round(phase * 256.0)) & 0xFF);
    } else {
      duties.at(tr.id()).duty = static_cast<uint16_t>(static_cast<double>(tr.cycle()) * std::asin(std::clamp(amp, 0.0, 1.0)) / driver::pi);
      phases.at(tr.id()).phase = static_cast<uint16_t>(
          rem_euclid(static_cast<int32_t>(std::round(phase * static_cast<double>(tr.cycle()))), static_cast<int32_t>(tr.cycle())));
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
    if (_legacy_mode)
      normal_legacy_header(tx);
    else
      normal_header(tx);
  }

  static void pack_body(bool& phase_sent, bool& duty_sent, const D& drives, driver::TxDatagram& tx) noexcept {
    if (_legacy_mode) {
      normal_legacy_body(drives.legacy_drives.data(), tx);
      phase_sent = true;
      duty_sent = true;
    } else {
      if (!phase_sent) {
        normal_phase_body(drives.phases.data(), tx);
        phase_sent = true;
      } else {
        normal_duty_body(drives.duties.data(), tx);
        duty_sent = true;
      }
    }
  }

  void set_cycle(const uint16_t cycle) noexcept {
    if (!_legacy_mode) _cycle = cycle;
  }

  void set_frequency(const double freq) noexcept {
    const auto cycle = static_cast<uint16_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / freq));
    set_cycle(cycle);
  }

  static bool& legacy_mode() { return _legacy_mode; }

 private:
  inline static bool _legacy_mode = true;
  uint16_t _cycle;
};

}  // namespace autd3::core
