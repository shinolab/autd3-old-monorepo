// File: normal_phase_transducer.hpp
// Project: geometry
// Created Date: 30/05/2022
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

#include "autd3/core/interface.hpp"
#include "autd3/core/utils.hpp"
#include "autd3/driver/cpu/operation.hpp"
#include "autd3/driver/fpga/defined.hpp"
#include "transducer.hpp"

namespace autd3::core {

/**
 * @brief DriveData for NormalPhaseTransducer
 */
template <typename T>
struct NormalPhaseDriveData final : DriveData<T> {
  void init(const size_t size) override { phases.resize(size, driver::Phase{0x0000}); }
  void set_drive(const T& tr, const double phase, double) override { phases.at(tr.id()).set(phase, tr.cycle()); }
  void copy_from(size_t idx, const typename T::D& src) override {
    auto ps = src.phases.data() + idx * driver::NUM_TRANS_IN_UNIT;
    const auto pd = phases.data() + idx * driver::NUM_TRANS_IN_UNIT;
    std::memcpy(pd, ps, sizeof(driver::Phase) * driver::NUM_TRANS_IN_UNIT);
  }

  std::vector<driver::Phase> phases{};
};

/**
 * \brief Transduce with variable frequency (phase only)
 */
struct NormalPhaseTransducer final : Transducer<NormalPhaseDriveData<NormalPhaseTransducer>> {
  NormalPhaseTransducer(const size_t id, Vector3 pos, Vector3 x_direction, Vector3 y_direction, Vector3 z_direction) noexcept
      : Transducer(id, std::move(pos), std::move(x_direction), std::move(y_direction), std::move(z_direction)), _cycle(4096) {}
  ~NormalPhaseTransducer() override = default;
  NormalPhaseTransducer(const NormalPhaseTransducer& v) noexcept = default;
  NormalPhaseTransducer& operator=(const NormalPhaseTransducer& obj) = default;
  NormalPhaseTransducer(NormalPhaseTransducer&& obj) = default;
  NormalPhaseTransducer& operator=(NormalPhaseTransducer&& obj) = default;

  [[nodiscard]] uint16_t cycle() const noexcept override { return _cycle; }
  [[nodiscard]] double frequency() const noexcept override { return static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(_cycle); }
  [[nodiscard]] double wavelength(const double sound_speed) const noexcept override { return sound_speed * 1e3 / frequency(); }
  [[nodiscard]] double wavenumber(const double sound_speed) const noexcept override { return 2.0 * driver::pi * frequency() / (sound_speed * 1e3); }

  static void pack_header(driver::TxDatagram& tx) noexcept { normal_header(tx); }

  static void pack_body(bool& phase_sent, bool& duty_sent, const D& drives, driver::TxDatagram& tx) noexcept {
    normal_phase_body(drives.phases.data(), tx);
    phase_sent = true;
    duty_sent = true;
  }

  void set_cycle(const uint16_t cycle) noexcept { _cycle = cycle; }

  void set_frequency(const double freq) noexcept {
    const auto cycle = static_cast<uint16_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / freq));
    set_cycle(cycle);
  }

 private:
  uint16_t _cycle;
};

/**
 * @brief Amplitude configuration for NormalPhaseTransducer.
 */
struct Amplitudes final : DatagramBody<NormalPhaseTransducer> {
  explicit Amplitudes(const Geometry<NormalPhaseTransducer>& geometry, const double amp = 1.0) : _sent(false) {
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

  void pack(const Geometry<NormalPhaseTransducer>&, driver::TxDatagram& tx) override {
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
