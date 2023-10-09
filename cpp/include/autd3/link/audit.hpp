// File: audit.hpp
// Project: link
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <utility>

#include "autd3/internal/controller.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

class Audit final {
  internal::native_methods::LinkPtr _ptr;

 public:
  class Builder final : internal::LinkBuilder {
    friend class Audit;

    internal::native_methods::LinkAuditBuilderPtr _ptr;

    Builder() : LinkBuilder(), _ptr(internal::native_methods::AUTDLinkAudit()) {}

   public:
    internal::native_methods::LinkBuilderPtr ptr() const override { return internal::native_methods::AUTDLinkAuditIntoBuilder(_ptr); }

    template <typename Rep, typename Period>
    Builder with_timeout(const std::chrono::duration<Rep, Period> timeout) {
      const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
      _ptr = AUTDLinkAuditWithTimeout(_ptr, static_cast<uint64_t>(ns));
      return std::move(*this);
    }
  };

  static Builder builder() { return Builder(); }

  explicit Audit(internal::native_methods::LinkPtr ptr) : _ptr(ptr) {}

  void down() { AUTDLinkAuditDown(_ptr); }

  bool is_open() { return AUTDLinkAuditIsOpen(_ptr); }

  std::uint64_t last_timeout_ns() { return AUTDLinkAuditLastTimeoutNs(_ptr); }

  void up() { AUTDLinkAuditUp(_ptr); }

  void break_down() { AUTDLinkAuditBreakDown(_ptr); }

  void update(const size_t idx) { AUTDLinkAuditCpuUpdate(_ptr, static_cast<std::uint32_t>(idx)); }

  int fpga_flags(const size_t idx) { return AUTDLinkAuditCpuFpgaFlags(_ptr, static_cast<std::uint32_t>(idx)); }

  bool is_legacy(const size_t idx) { return AUTDLinkAuditFpgaIsLegacyMode(_ptr, static_cast<std::uint32_t>(idx)); }

  int silencer_step(const size_t idx) { return AUTDLinkAuditFpgaSilencerStep(_ptr, static_cast<std::uint32_t>(idx)); }

  void assert_thermal_sensor(const size_t idx) { AUTDLinkAuditFpgaAssertThermalSensor(_ptr, static_cast<std::uint32_t>(idx)); }

  void deassert_thermal_sensor(const size_t idx) { AUTDLinkAuditFpgaDeassertThermalSensor(_ptr, static_cast<std::uint32_t>(idx)); }

  std::vector<std::uint8_t> modulation(const size_t idx) {
    const auto n = AUTDLinkAuditFpgaModulationCycle(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint8_t> buf(n);
    AUTDLinkAuditFpgaModulation(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  std::uint32_t modulation_frequency_division(const size_t idx) {
    return AUTDLinkAuditFpgaModulationFrequencyDivision(_ptr, static_cast<std::uint32_t>(idx));
  }

  std::vector<std::uint16_t> cycles(const size_t idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    AUTDLinkAuditFpgaCycles(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  std::vector<std::uint16_t> mod_delays(const size_t idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    AUTDLinkAuditFpgaModDelays(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  std::vector<std::int16_t> duty_filters(const size_t idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::int16_t> buf(n);
    AUTDLinkAuditFpgaDutyFilters(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  std::vector<std::int16_t> phase_filters(const size_t idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::int16_t> buf(n);
    AUTDLinkAuditFpgaPhaseFilters(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  std::pair<std::vector<std::uint16_t>, std::vector<std::uint16_t>> duties_and_phases(const size_t idx, const int stm_idx) {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> duties(n);
    std::vector<std::uint16_t> phases(n);
    AUTDLinkAuditFpgaDutiesAndPhases(_ptr, static_cast<std::uint32_t>(idx), static_cast<std::uint32_t>(stm_idx), duties.data(), phases.data());
    return std::make_pair(duties, phases);
  }

  std::uint32_t stm_cycle(const size_t idx) { return AUTDLinkAuditFpgaStmCycle(_ptr, static_cast<std::uint32_t>(idx)); }

  bool is_stm_gain_mode(const size_t idx) { return AUTDLinkAuditFpgaIsStmGainMode(_ptr, static_cast<std::uint32_t>(idx)); }

  std::uint32_t stm_frequency_division(const size_t idx) { return AUTDLinkAuditFpgaStmFrequencyDivision(_ptr, static_cast<std::uint32_t>(idx)); }

  int stm_start_idx(const size_t idx) { return AUTDLinkAuditFpgaStmStartIdx(_ptr, static_cast<std::uint32_t>(idx)); }

  int stm_finish_idx(const size_t idx) { return AUTDLinkAuditFpgaStmFinishIdx(_ptr, static_cast<std::uint32_t>(idx)); }
};

}  // namespace autd3::link
