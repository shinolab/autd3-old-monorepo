// File: audit.hpp
// Project: link
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <utility>

#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/link/log.hpp"

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

  static void down(internal::Controller& cnt) { internal::native_methods::AUTDLinkAuditDown(internal::native_methods::AUTDAuditLinkGet(cnt._ptr)); }

  static bool is_open(internal::Controller& cnt) { return internal::native_methods::AUTDLinkAuditIsOpen(internal::native_methods::AUTDAuditLinkGet(cnt._ptr)); }

  static std::uint64_t last_timeout_ns(internal::Controller& cnt) {
    return internal::native_methods::AUTDLinkAuditLastTimeoutNs(internal::native_methods::AUTDAuditLinkGet(cnt._ptr));
  }

  static void up(internal::Controller& cnt) { internal::native_methods::AUTDLinkAuditUp(internal::native_methods::AUTDAuditLinkGet(cnt._ptr)); }

  static void break_down(internal::Controller& cnt) { internal::native_methods::AUTDLinkAuditBreakDown(internal::native_methods::AUTDAuditLinkGet(cnt._ptr)); }

  static void update(internal::Controller& cnt, size_t idx) {
    internal::native_methods::AUTDLinkAuditCpuUpdate(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static int fpga_flags(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditCpuFpgaFlags(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static bool is_legacy(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditFpgaIsLegacyMode(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                   static_cast<std::uint32_t>(idx));
  }

  static int silencer_step(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditFpgaSilencerStep(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                   static_cast<std::uint32_t>(idx));
  }

  static void assert_thermal_sensor(internal::Controller& cnt, size_t idx) {
    internal::native_methods::AUTDLinkAuditFpgaAssertThermalSensor(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                   static_cast<std::uint32_t>(idx));
  }

  static void deassert_thermal_sensor(internal::Controller& cnt, size_t idx) {
    internal::native_methods::AUTDLinkAuditFpgaDeassertThermalSensor(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                     static_cast<std::uint32_t>(idx));
  }

  static std::vector<std::uint8_t> modulation(internal::Controller& cnt, size_t idx) {
    auto n = internal::native_methods::AUTDLinkAuditFpgaModulationCycle(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                        static_cast<std::uint32_t>(idx));
    std::vector<std::uint8_t> buf(n);
    internal::native_methods::AUTDLinkAuditFpgaModulation(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx),
                                                          buf.data());
    return buf;
  }

  static std::uint32_t modulation_frequency_division(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditFpgaModulationFrequencyDivision(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                                  static_cast<std::uint32_t>(idx));
  }

  static std::vector<std::uint16_t> cycles(internal::Controller& cnt, size_t idx) {
    auto n = internal::native_methods::AUTDLinkAuditCpuNumTransducers(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                      static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    internal::native_methods::AUTDLinkAuditFpgaCycles(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx),
                                                      buf.data());
    return buf;
  }

  static std::vector<std::uint16_t> mod_delays(internal::Controller& cnt, size_t idx) {
    auto n = internal::native_methods::AUTDLinkAuditCpuNumTransducers(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                      static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    internal::native_methods::AUTDLinkAuditFpgaModDelays(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx),
                                                         buf.data());
    return buf;
  }

  static std::vector<std::int16_t> duty_filters(internal::Controller& cnt, size_t idx) {
    auto n = internal::native_methods::AUTDLinkAuditCpuNumTransducers(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                      static_cast<std::uint32_t>(idx));
    std::vector<std::int16_t> buf(n);
    internal::native_methods::AUTDLinkAuditFpgaDutyFilters(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx),
                                                           buf.data());
    return buf;
  }

  static std::vector<std::int16_t> phase_filters(internal::Controller& cnt, size_t idx) {
    auto n = internal::native_methods::AUTDLinkAuditCpuNumTransducers(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                      static_cast<std::uint32_t>(idx));
    std::vector<std::int16_t> buf(n);
    internal::native_methods::AUTDLinkAuditFpgaPhaseFilters(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx),
                                                            buf.data());
    return buf;
  }

  static std::pair<std::vector<std::uint16_t>, std::vector<std::uint16_t>> duties_and_phases(internal::Controller& cnt, size_t idx, int stmIdx) {
    auto n = internal::native_methods::AUTDLinkAuditCpuNumTransducers(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                      static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> duties(n);
    std::vector<std::uint16_t> phases(n);
    internal::native_methods::AUTDLinkAuditFpgaDutiesAndPhases(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx),
                                                               static_cast<std::uint32_t>(stmIdx), duties.data(), phases.data());
    return std::make_pair(duties, phases);
  }

  static std::uint32_t stm_cycle(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditFpgaStmCycle(internal::native_methods::AUTDAuditLinkGet(cnt._ptr), static_cast<std::uint32_t>(idx));
  }

  static bool is_stm_gain_mode(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditFpgaIsStmGainMode(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                    static_cast<std::uint32_t>(idx));
  }

  static std::uint32_t stm_frequency_division(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditFpgaStmFrequencyDivision(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                           static_cast<std::uint32_t>(idx));
  }

  static int stm_start_idx(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditFpgaStmStartIdx(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                  static_cast<std::uint32_t>(idx));
  }

  static int stm_finish_idx(internal::Controller& cnt, size_t idx) {
    return internal::native_methods::AUTDLinkAuditFpgaStmFinishIdx(internal::native_methods::AUTDAuditLinkGet(cnt._ptr),
                                                                   static_cast<std::uint32_t>(idx));
  }
};

}  // namespace autd3::link
