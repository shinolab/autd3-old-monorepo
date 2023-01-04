// File: mode.cpp
// Project: core
// Created Date: 22/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/core/mode.hpp"

#include "../spdlog.hpp"

namespace autd3::core {

bool LegacyMode::pack_stm_gain_header(driver::TxDatagram& tx) const noexcept { return driver::GainSTMLegacyHeader().pack(tx); }

bool LegacyMode::pack_stm_gain_body(size_t& sent, bool&, const uint32_t freq_div, const std::vector<std::vector<driver::Drive>>& gains,
                                    const Geometry& geometry, const driver::GainSTMMode mode, const std::optional<uint16_t> start_idx,
                                    const std::optional<uint16_t> finish_idx, driver::TxDatagram& tx) const {
  return driver::GainSTMLegacyBody().drives(gains).sent(&sent).freq_div(freq_div).mode(mode).start_idx(start_idx).finish_idx(finish_idx).pack(tx);
}

bool NormalMode::pack_stm_gain_header(driver::TxDatagram& tx) const noexcept { driver->gain_stm_normal_header(tx); }
bool NormalMode::pack_stm_gain_body(size_t& sent, bool& next_duty, const uint32_t freq_div, const std::vector<std::vector<driver::Drive>>& gains,
                                    const Geometry& geometry, const driver::GainSTMMode mode, const std::optional<uint16_t> start_idx,
                                    const std::optional<uint16_t> finish_idx, driver::TxDatagram& tx) const {
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

bool NormalPhaseMode::pack_stm_gain_header(driver::TxDatagram& tx) const noexcept { driver->gain_stm_normal_header(tx); }
bool NormalPhaseMode::pack_stm_gain_body(size_t& sent, bool&, const uint32_t freq_div, const std::vector<std::vector<driver::Drive>>& gains,
                                         const Geometry& geometry, driver::GainSTMMode, const std::optional<uint16_t> start_idx,
                                         const std::optional<uint16_t> finish_idx, driver::TxDatagram& tx) const {
  const auto cycles = geometry.cycles();
  return driver->gain_stm_normal_phase(gains, cycles, sent++, freq_div, driver::GainSTMMode::PhaseFull, start_idx, finish_idx, tx);
}

}  // namespace autd3::core
