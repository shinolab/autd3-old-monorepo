// File: mode.cpp
// Project: core
// Created Date: 22/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/core/mode.hpp"

#include "../spdlog.hpp"

namespace autd3::core {
bool LegacyMode::pack_sync(const std::unique_ptr<const driver::Driver>& driver, const Geometry& geometry, driver::TxDatagram& tx) const {
  const auto cycles = geometry.cycles();
  if (std::any_of(cycles.begin(), cycles.end(), [](const uint16_t cycle) { return cycle != 4096; })) {
    spdlog::error("Cannot change frequency in LegacyMode.");
    return false;
  }
  driver->sync(cycles, tx);
  return true;
}
void LegacyMode::pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept {
  driver->normal_legacy_header(tx);
}
void LegacyMode::pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                                const std::vector<driver::Drive>& drives, const Geometry&, driver::TxDatagram& tx) const {
  driver->normal_legacy_body(drives, tx);
  phase_sent = true;
  duty_sent = true;
}
void LegacyMode::pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept {
  driver->gain_stm_legacy_header(tx);
}
bool LegacyMode::pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool&, const uint32_t freq_div,
                                    const std::vector<std::vector<driver::Drive>>& gains, const Geometry& geometry, const driver::GainSTMMode mode,
                                    const std::optional<uint16_t> start_idx, const std::optional<uint16_t> finish_idx, driver::TxDatagram& tx) const {
  return driver->gain_stm_legacy_body(gains, sent, freq_div, mode, start_idx, finish_idx, tx);
}
std::unique_ptr<LegacyMode> LegacyMode::create() noexcept { return std::make_unique<LegacyMode>(); }
bool NormalMode::pack_sync(const std::unique_ptr<const driver::Driver>& driver, const Geometry& geometry, driver::TxDatagram& tx) const {
  const auto cycles = geometry.cycles();
  driver->sync(cycles, tx);
  return true;
}
void NormalMode::pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept {
  driver->normal_header(tx);
}
void NormalMode::pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                                const std::vector<driver::Drive>& drives, const Geometry& geometry, driver::TxDatagram& tx) const {
  const auto cycles = geometry.cycles();
  if (!phase_sent) {
    driver->normal_phase_body(drives, cycles, tx);
    phase_sent = true;
  } else {
    driver->normal_duty_body(drives, cycles, tx);
    duty_sent = true;
  }
}
void NormalMode::pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept {
  driver->gain_stm_normal_header(tx);
}
bool NormalMode::pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool& next_duty, const uint32_t freq_div,
                                    const std::vector<std::vector<driver::Drive>>& gains, const Geometry& geometry, const driver::GainSTMMode mode,
                                    const std::optional<uint16_t> start_idx, const std::optional<uint16_t> finish_idx, driver::TxDatagram& tx) const {
  const auto cycles = geometry.cycles();
  if (sent == 0) return driver->gain_stm_normal_phase(gains, cycles, sent++, freq_div, mode, start_idx, finish_idx, tx);

  switch (mode) {
    case driver::GainSTMMode::PhaseDutyFull:
      next_duty = !next_duty;
      return next_duty ? driver->gain_stm_normal_phase(gains, cycles, sent, freq_div, mode, start_idx, finish_idx, tx)
                       : driver->gain_stm_normal_duty(gains, cycles, sent++, freq_div, mode, start_idx, finish_idx, tx);
    case driver::GainSTMMode::PhaseFull:
      return driver->gain_stm_normal_phase(gains, cycles, sent++, freq_div, mode, start_idx, finish_idx, tx);
    case driver::GainSTMMode::PhaseHalf:
      spdlog::error("This mode is not supported");
      return false;
  }
  return false;
}
std::unique_ptr<NormalMode> NormalMode::create() noexcept { return std::make_unique<NormalMode>(); }
bool NormalPhaseMode::pack_sync(const std::unique_ptr<const driver::Driver>& driver, const Geometry& geometry, driver::TxDatagram& tx) const {
  const auto cycles = geometry.cycles();
  driver->sync(cycles, tx);
  return true;
}
void NormalPhaseMode::pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept {
  driver->normal_header(tx);
}
void NormalPhaseMode::pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                                     const std::vector<driver::Drive>& drives, const Geometry& geometry, driver::TxDatagram& tx) const {
  const auto cycles = geometry.cycles();
  driver->normal_phase_body(drives, cycles, tx);
  phase_sent = true;
  duty_sent = true;
}
void NormalPhaseMode::pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept {
  driver->gain_stm_normal_header(tx);
}
bool NormalPhaseMode::pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool&, const uint32_t freq_div,
                                         const std::vector<std::vector<driver::Drive>>& gains, const Geometry& geometry, driver::GainSTMMode,
                                         const std::optional<uint16_t> start_idx, const std::optional<uint16_t> finish_idx,
                                         driver::TxDatagram& tx) const {
  const auto cycles = geometry.cycles();
  return driver->gain_stm_normal_phase(gains, cycles, sent++, freq_div, driver::GainSTMMode::PhaseFull, start_idx, finish_idx, tx);
}
std::unique_ptr<NormalPhaseMode> NormalPhaseMode::create() noexcept { return std::make_unique<NormalPhaseMode>(); }
}  // namespace autd3::core
