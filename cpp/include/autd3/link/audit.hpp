// File: audit.hpp
// Project: link
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <utility>

#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

class Audit : public internal::Link {
 public:
  Audit() : Link(internal::native_methods::AUTDLinkAudit()) {}

  template <typename Rep, typename Period>
  Audit with_timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _ptr = AUTDLinkAuditWithTimeout(_ptr, static_cast<uint64_t>(ns));
    return std::move(*this);
  }

  static void down(const internal::Controller& cnt) { AUTDLinkAuditDown(AUTDAuditLinkGet(cnt._ptr)); }

  static bool is_open(const internal::Controller& cnt) { return AUTDLinkAuditIsOpen(AUTDAuditLinkGet(cnt._ptr)); }

  static std::uint64_t last_timeout_ns(const internal::Controller& cnt) { return AUTDLinkAuditLastTimeoutNs(AUTDAuditLinkGet(cnt._ptr)); }

  static void up(const internal::Controller& cnt) { AUTDLinkAuditUp(AUTDAuditLinkGet(cnt._ptr)); }

  static void break_down(const internal::Controller& cnt) { AUTDLinkAuditBreakDown(AUTDAuditLinkGet(cnt._ptr)); }

  static void update(const internal::Controller& cnt, const size_t idx) {
    AUTDLinkAuditCpuUpdate(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static int fpga_flags(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditCpuFpgaFlags(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static bool is_legacy(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditFpgaIsLegacyMode(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static int silencer_step(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditFpgaSilencerStep(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static void assert_thermal_sensor(const internal::Controller& cnt, const size_t idx) {
    AUTDLinkAuditFpgaAssertThermalSensor(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static void deassert_thermal_sensor(const internal::Controller& cnt, const size_t idx) {
    AUTDLinkAuditFpgaDeassertThermalSensor(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static std::vector<std::uint8_t> modulation(const internal::Controller& cnt, const size_t idx) {
    const auto n = AUTDLinkAuditFpgaModulationCycle(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
    std::vector<std::uint8_t> buf(n);
    AUTDLinkAuditFpgaModulation(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  static std::uint32_t modulation_frequency_division(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditFpgaModulationFrequencyDivision(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static std::vector<std::uint16_t> cycles(const internal::Controller& cnt, const size_t idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    AUTDLinkAuditFpgaCycles(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  static std::vector<std::uint16_t> mod_delays(const internal::Controller& cnt, const size_t idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    AUTDLinkAuditFpgaModDelays(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  static std::vector<std::int16_t> duty_filters(const internal::Controller& cnt, const size_t idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
    std::vector<std::int16_t> buf(n);
    AUTDLinkAuditFpgaDutyFilters(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  static std::vector<std::int16_t> phase_filters(const internal::Controller& cnt, const size_t idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
    std::vector<std::int16_t> buf(n);
    AUTDLinkAuditFpgaPhaseFilters(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  static std::pair<std::vector<std::uint16_t>, std::vector<std::uint16_t>> duties_and_phases(const internal::Controller& cnt, const size_t idx,
                                                                                             const int stm_idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> duties(n);
    std::vector<std::uint16_t> phases(n);
    AUTDLinkAuditFpgaDutiesAndPhases(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx), static_cast<std::uint32_t>(stm_idx), duties.data(),
                                     phases.data());
    return std::make_pair(duties, phases);
  }

  static std::uint32_t stm_cycle(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditFpgaStmCycle(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static bool is_stm_gain_mode(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditFpgaIsStmGainMode(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static std::uint32_t stm_frequency_division(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditFpgaStmFrequencyDivision(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static int stm_start_idx(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditFpgaStmStartIdx(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static int stm_finish_idx(const internal::Controller& cnt, const size_t idx) {
    return AUTDLinkAuditFpgaStmFinishIdx(AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }
};

}  // namespace autd3::link
