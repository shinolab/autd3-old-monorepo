// File: audit.hpp
// Project: link
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <utility>

#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {
class ControllerBuilder;
}

namespace autd3::link {

class Audit final {
  internal::native_methods::LinkPtr _ptr;

  explicit Audit(const internal::native_methods::LinkPtr ptr) : _ptr(ptr) {}

 public:
  class Builder final {
    friend class Audit;
    friend class internal::ControllerBuilder;

    internal::native_methods::LinkAuditBuilderPtr _ptr;

    Builder() : _ptr(internal::native_methods::AUTDLinkAudit()) {}

    [[nodiscard]] static Audit resolve_link(const internal::native_methods::LinkPtr link) { return Audit{link}; }

   public:
    using Link = Audit;

    [[nodiscard]] internal::native_methods::LinkBuilderPtr ptr() const { return AUTDLinkAuditIntoBuilder(_ptr); }

    template <typename Rep, typename Period>
    Builder with_timeout(const std::chrono::duration<Rep, Period> timeout) {
      const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
      _ptr = AUTDLinkAuditWithTimeout(_ptr, static_cast<uint64_t>(ns));
      return std::move(*this);
    }
  };

  static Builder builder() { return {}; }

  void down() const { AUTDLinkAuditDown(_ptr); }

  [[nodiscard]] bool is_open() const { return AUTDLinkAuditIsOpen(_ptr); }

  [[nodiscard]] bool is_force_fan(const size_t idx) const { return AUTDLinkAuditFpgaIsForceFan(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] std::uint64_t last_timeout_ns() const { return AUTDLinkAuditLastTimeoutNs(_ptr); }

  void up() const { AUTDLinkAuditUp(_ptr); }

  void break_down() const { AUTDLinkAuditBreakDown(_ptr); }

  void update(const size_t idx) const { AUTDLinkAuditCpuUpdate(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] int silencer_step_intensity(const size_t idx) const {
    return AUTDLinkAuditFpgaSilencerStepIntensity(_ptr, static_cast<std::uint32_t>(idx));
  }
  [[nodiscard]] int silencer_step_phase(const size_t idx) const { return AUTDLinkAuditFpgaSilencerStepPhase(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] uint8_t debug_output_idx(const size_t idx) const { return AUTDLinkAuditFpgaDebugOutputIdx(_ptr, static_cast<std::uint32_t>(idx)); }

  void assert_thermal_sensor(const size_t idx) const { AUTDLinkAuditFpgaAssertThermalSensor(_ptr, static_cast<std::uint32_t>(idx)); }

  void deassert_thermal_sensor(const size_t idx) const { AUTDLinkAuditFpgaDeassertThermalSensor(_ptr, static_cast<std::uint32_t>(idx)); }

  [[nodiscard]] std::vector<std::uint8_t> modulation(const size_t idx) const {
    const auto n = AUTDLinkAuditFpgaModulationCycle(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint8_t> buf(n);
    AUTDLinkAuditFpgaModulation(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }  // LCOV_EXCL_LINE

  [[nodiscard]] std::uint32_t modulation_frequency_division(const size_t idx) const {
    return AUTDLinkAuditFpgaModulationFrequencyDivision(_ptr, static_cast<std::uint32_t>(idx));
  }

  [[nodiscard]] std::vector<std::uint16_t> mod_delays(const size_t idx) const {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint16_t> buf(n);
    AUTDLinkAuditFpgaModDelays(_ptr, static_cast<std::uint32_t>(idx), buf.data());
    return buf;
  }  // LCOV_EXCL_LINE

  [[nodiscard]] std::pair<std::vector<std::uint8_t>, std::vector<std::uint8_t>> intensities_and_phases(const size_t idx, const int stm_idx) const {
    const auto n = AUTDLinkAuditCpuNumTransducers(_ptr, static_cast<std::uint32_t>(idx));
    std::vector<std::uint8_t> duties(n);
    std::vector<std::uint8_t> phases(n);
    AUTDLinkAuditFpgaIntensitiesAndPhases(_ptr, static_cast<std::uint32_t>(idx), static_cast<std::uint32_t>(stm_idx), duties.data(), phases.data());
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
