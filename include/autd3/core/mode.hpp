// File: mode.hpp
// Project: core
// Created Date: 28/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/cpu/operation.hpp"
#include "autd3/driver/fpga/defined.hpp"

namespace autd3::core {

class Mode {
 public:
  virtual void pack_gain_header(driver::TxDatagram& tx) const = 0;
  virtual void pack_gain_body(bool& phase_sent, bool& duty_sent, const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const = 0;

  virtual void pack_stm_gain_header(driver::TxDatagram& tx) const = 0;
  virtual void pack_stm_gain_body(size_t& sent, bool& next_duty, uint32_t freq_div, const std::vector<std::vector<driver::Drive>>& gains,
                                  driver::GainSTMMode mode, driver::TxDatagram& tx) const = 0;
};

class LegacyMode : public Mode {
  void pack_gain_header(driver::TxDatagram& tx) const noexcept override { normal_legacy_header(tx); }
  void pack_gain_body(bool& phase_sent, bool& duty_sent, const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override {
    normal_legacy_body(drives, tx);
    phase_sent = true;
    duty_sent = true;
  }

  void pack_stm_gain_header(driver::TxDatagram& tx) const noexcept override { gain_stm_legacy_header(tx); }

  void pack_stm_gain_body(size_t& sent, bool&, uint32_t freq_div, const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode,
                          driver::TxDatagram& tx) const override {
    const auto is_first_frame = sent == 0;

    if (is_first_frame) {
      gain_stm_legacy_body({}, is_first_frame, freq_div, false, mode, tx);
      sent += 1;
      return;
    }

    bool is_last_frame;
    const std::vector<driver::Drive>*p1, *p2, *p3, *p4;
    switch (mode) {
      case driver::GainSTMMode::PhaseDutyFull:
        is_last_frame = sent + 1 == gains.size() + 1;
        gain_stm_legacy_body({&gains.at(sent - 1)}, is_first_frame, freq_div, is_last_frame, mode, tx);
        sent += 1;
        break;
      case driver::GainSTMMode::PhaseFull:
        is_last_frame = sent + 2 >= gains.size() + 1;
        p1 = &gains.at(sent - 1);
        p2 = sent + 1 - 1 < gains.size() ? &gains.at(sent + 1 - 1) : nullptr;
        gain_stm_legacy_body({p1, p2}, is_first_frame, freq_div, is_last_frame, mode, tx);
        sent += 2;
        break;
      case driver::GainSTMMode::PhaseHalf:
        is_last_frame = sent + 4 >= gains.size() + 1;
        p1 = &gains.at(sent - 1);
        p2 = sent + 1 - 1 < gains.size() ? &gains.at(sent + 1 - 1) : nullptr;
        p3 = sent + 2 - 1 < gains.size() ? &gains.at(sent + 2 - 1) : nullptr;
        p4 = sent + 3 - 1 < gains.size() ? &gains.at(sent + 3 - 1) : nullptr;
        gain_stm_legacy_body({p1, p2, p3, p4}, is_first_frame, freq_div, is_last_frame, mode, tx);
        sent += 4;
        break;
    }
  }
};

class NormalMode : public Mode {
  void pack_gain_header(driver::TxDatagram& tx) const noexcept override { normal_header(tx); }
  void pack_gain_body(bool& phase_sent, bool& duty_sent, const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override {
    if (!phase_sent) {
      normal_phase_body(drives, tx);
      phase_sent = true;
    } else {
      normal_duty_body(drives, tx);
      duty_sent = true;
    }
  }

  void pack_stm_gain_header(driver::TxDatagram& tx) const noexcept override { gain_stm_normal_header(tx); }

  void pack_stm_gain_body(size_t& sent, bool& next_duty, uint32_t freq_div, const std::vector<std::vector<driver::Drive>>& gains,
                          driver::GainSTMMode mode, driver::TxDatagram& tx) const override {
    const auto is_first_frame = sent == 0;

    if (is_first_frame) {
      gain_stm_normal_phase({}, is_first_frame, freq_div, mode, false, tx);
      sent += 1;
      return;
    }

    const auto is_last_frame = sent + 1 == gains.size() + 1;

    switch (mode) {
      case driver::GainSTMMode::PhaseDutyFull:
        if (next_duty) {
          gain_stm_normal_duty(gains.at(sent - 1), is_last_frame, tx);
          sent += 1;
        } else {
          gain_stm_normal_phase(gains.at(sent - 1), is_first_frame, freq_div, mode, is_last_frame, tx);
        }
        next_duty = !next_duty;
        break;
      case driver::GainSTMMode::PhaseFull:
        gain_stm_normal_phase(gains.at(sent - 1), is_first_frame, freq_div, mode, is_last_frame, tx);
        sent += 1;
        break;
      default:
        throw std::runtime_error("This mode is not supported");
        break;
    }
  }
};

class NormalPhaseMode : public Mode {
  void pack_gain_header(driver::TxDatagram& tx) const noexcept override { normal_header(tx); }
  void pack_gain_body(bool& phase_sent, bool& duty_sent, const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override {
    normal_phase_body(drives, tx);
    phase_sent = true;
    duty_sent = true;
  }

  void pack_stm_gain_header(driver::TxDatagram& tx) const noexcept override { gain_stm_normal_header(tx); }

  void pack_stm_gain_body(size_t& sent, bool&, uint32_t freq_div, const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode,
                          driver::TxDatagram& tx) const override {
    const auto is_first_frame = sent == 0;

    if (is_first_frame) {
      gain_stm_normal_phase({}, is_first_frame, freq_div, driver::GainSTMMode::PhaseFull, false, tx);
      sent += 1;
      return;
    }

    const auto is_last_frame = sent + 1 == gains.size() + 1;

    switch (mode) {
      case driver::GainSTMMode::PhaseDutyFull:
      case driver::GainSTMMode::PhaseFull:
        gain_stm_normal_phase(gains.at(sent - 1), is_first_frame, freq_div, driver::GainSTMMode::PhaseFull, is_last_frame, tx);
        sent += 1;
        break;
      default:
        throw std::runtime_error("This mode is not supported");
        break;
    }
  }
};

}  // namespace autd3::core
