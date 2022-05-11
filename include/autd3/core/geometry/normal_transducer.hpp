// File: normal_transducer.hpp
// Project: geometry
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <gsl/gsl>
#include <numbers>
#include <utility>
#include <vector>

#include "autd3/core/utils.hpp"
#include "autd3/driver/cpu/operation.hpp"
#include "autd3/driver/fpga/defined.hpp"
#include "transducer.hpp"

namespace autd3::core {

template <typename T>
struct NormalDriveData final : DriveData<T> {
  void init(const size_t size) override {
    duties.resize(size, driver::Duty{0x0000});
    phases.resize(size, driver::Phase{0x0000});
  }
  void set_drive(const T& tr, const double phase, const double amp) override {
    duties.at(tr.id()).duty = gsl::narrow_cast<uint16_t>(static_cast<double>(tr.cycle()) * std::asin(amp) / std::numbers::pi);
    phases.at(tr.id()).phase = gsl::narrow_cast<uint16_t>(
        rem_euclid(static_cast<int32_t>(std::round(phase * static_cast<double>(tr.cycle()))), static_cast<int32_t>(tr.cycle())));
  }

  std::vector<driver::Duty> duties{};
  std::vector<driver::Phase> phases{};
};

struct NormalTransducer final : Transducer<NormalDriveData<NormalTransducer>> {
  NormalTransducer(const size_t id, Vector3 pos, Vector3 x_direction, Vector3 y_direction, Vector3 z_direction) noexcept
      : Transducer(id, std::move(pos), std::move(x_direction), std::move(y_direction), std::move(z_direction)), _cycle(4096) {}
  ~NormalTransducer() override = default;
  NormalTransducer(const NormalTransducer& v) noexcept = delete;
  NormalTransducer& operator=(const NormalTransducer& obj) = delete;
  NormalTransducer(NormalTransducer&& obj) = default;
  NormalTransducer& operator=(NormalTransducer&& obj) = default;

  [[nodiscard]] uint16_t cycle() const noexcept override { return _cycle; }
  [[nodiscard]] double frequency() const noexcept override { return static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(_cycle); }
  [[nodiscard]] double wavelength(const double sound_speed) const noexcept override { return sound_speed * 1e3 / 40e3; }
  [[nodiscard]] double wavenumber(const double sound_speed) const noexcept override { return 2.0 * std::numbers::pi * 40e3 / (sound_speed * 1e3); }

  static void pack(const uint8_t msg_id, bool& phase_sent, bool& duty_sent, D& drives, driver::TxDatagram& tx) noexcept {
    if (!phase_sent) {
      normal_phase(msg_id, gsl::span{drives.phases}, tx);
      phase_sent = true;
    } else {
      normal_duty(msg_id, gsl::span{drives.duties}, tx);
      duty_sent = true;
    }
  }

  void set_cycle(const uint16_t cycle) noexcept { _cycle = cycle; }

  void set_frequency(const double freq) noexcept {
    const auto cycle = gsl::narrow_cast<uint16_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / freq));
    set_cycle(cycle);
  }

 private:
  uint16_t _cycle;
};

}  // namespace autd3::core
