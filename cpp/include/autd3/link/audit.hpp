// File: audit.hpp
// Project: link
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
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

class Audit final {
  internal::native_methods::LinkPtr _ptr;

 public:
  class Builder final : internal::LinkBuilder {
    friend class Audit;

    internal::native_methods::LinkAuditBuilderPtr _ptr;

    Builder() : LinkBuilder(), _ptr(internal::native_methods::AUTDLinkAudit()) {}

   public:
    [[nodiscard]] internal::native_methods::LinkBuilderPtr ptr() const override { return AUTDLinkAuditIntoBuilder(_ptr); }

    template <typename Rep, typename Period>
    Builder with_timeout(const std::chrono::duration<Rep, Period> timeout) {
      const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
      _ptr = AUTDLinkAuditWithTimeout(_ptr, static_cast<uint64_t>(ns));
      return std::move(*this);
    }
  };

  static Builder builder() { return {}; }

  explicit Audit(const internal::native_methods::LinkPtr ptr) : _ptr(ptr) {}

  void down() const { AUTDLinkAuditDown(_ptr); }

  [[nodiscard]] bool is_open() const { return AUTDLinkAuditIsOpen(_ptr); }

  [[nodiscard]] std::uint64_t last_timeout_ns() const { return AUTDLinkAuditLastTimeoutNs(_ptr); }

  void up() const { AUTDLinkAuditUp(_ptr); }

  void break_down() const { AUTDLinkAuditBreakDown(_ptr); }

  void update(const size_t idx) const { AUTDLinkAuditCpuUpdate(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] int fpga_flags(const size_t idx) const { return AUTDLinkAuditCpuFpgaFlags(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] bool is_legacy(const size_t idx) const { return AUTDLinkAuditFpgaIsLegacyMode(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] int silencer_step(const size_t idx) const { return AUTDLinkAuditFpgaSilencerStep(_ptr, static_cast<std::uint32_t>(idx)); }

  void assert_thermal_sensor(const size_t idx) const { AUTDLinkAuditFpgaAssertThermalSensor(_ptr, static_cast<std::uint32_t>(idx)); }

  void deassert_thermal_sensor(const size_t idx) const { AUTDLinkAuditFpgaDeassertThermalSensor(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] std::vector<std::uint8_t> modulation(const size_t idx) const {
    const auto n = AUTDLinkAuditFpgaModulationCycle(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint8_t> buf(n);
    AUTDLinkAuditFpgaModulation(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  [[nodiscard]] std::uint32_t modulation_frequency_division(const size_t idx) const {
    return AUTDLinkAuditFpgaModulationFrequencyDivision(_ptr, static_cast<std::uint32_t>(idx));
  }

  [[nodiscard]] std::vector<std::uint16_t> cycles(const size_t idx) const {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    AUTDLinkAuditFpgaCycles(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  [[nodiscard]] std::vector<std::uint16_t> mod_delays(const size_t idx) const {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    AUTDLinkAuditFpgaModDelays(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  [[nodiscard]] std::vector<std::int16_t> duty_filters(const size_t idx) const {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::int16_t> buf(n);
    AUTDLinkAuditFpgaDutyFilters(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  [[nodiscard]] std::vector<std::int16_t> phase_filters(const size_t idx) const {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::int16_t> buf(n);
    AUTDLinkAuditFpgaPhaseFilters(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }

  [[nodiscard]] std::pair<std::vector<std::uint16_t>, std::vector<std::uint16_t>> duties_and_phases(const size_t idx, const int stm_idx) const {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> duties(n);
    std::vector<std::uint16_t> phases(n);
    AUTDLinkAuditFpgaDutiesAndPhases(_ptr, static_cast<std::uint32_t>(idx), static_cast<std::uint32_t>(stm_idx), duties.data(), phases.data());
    return std::make_pair(duties, phases);
  }

  [[nodiscard]] std::uint32_t stm_cycle(const size_t idx) const { return AUTDLinkAuditFpgaStmCycle(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] bool is_stm_gain_mode(const size_t idx) const { return AUTDLinkAuditFpgaIsStmGainMode(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] std::uint32_t stm_frequency_division(const size_t idx) const {
    return AUTDLinkAuditFpgaStmFrequencyDivision(_ptr, static_cast<std::uint32_t>(idx));
  }

  [[nodiscard]] int stm_start_idx(const size_t idx) const { return AUTDLinkAuditFpgaStmStartIdx(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] int stm_finish_idx(const size_t idx) const { return AUTDLinkAuditFpgaStmFinishIdx(_ptr, static_cast<std::uint32_t>(idx)); }
};

}  // namespace autd3::link
