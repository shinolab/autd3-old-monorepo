// File: mode.hpp
// Project: core
// Created Date: 28/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/driver/driver.hpp"

namespace autd3::core {

class Mode {
 public:
  virtual void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const = 0;
  virtual void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                              const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const = 0;
  virtual void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const = 0;
  virtual void pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool& next_duty, uint32_t freq_div,
                                  const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode, driver::TxDatagram& tx) const = 0;
  virtual ~Mode() = default;
};

class LegacyMode : public Mode {
  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override {
    driver->normal_legacy_header(tx);
  }
  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override {
    driver->normal_legacy_body(drives, tx);
    phase_sent = true;
    duty_sent = true;
  }

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override {
    driver->gain_stm_legacy_header(tx);
  }

  void pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool&, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode, driver::TxDatagram& tx) const override {
    driver->gain_stm_legacy_body(gains, sent, freq_div, mode, tx);
  }

 public:
  ~LegacyMode() override = default;
  static std::unique_ptr<LegacyMode> create() { return std::make_unique<LegacyMode>(); }
};

class NormalMode : public Mode {
  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override {
    driver->normal_header(tx);
  }
  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override {
    if (!phase_sent) {
      driver->normal_phase_body(drives, tx);
      phase_sent = true;
    } else {
      driver->normal_duty_body(drives, tx);
      duty_sent = true;
    }
  }

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override {
    driver->gain_stm_normal_header(tx);
  }

  void pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool& next_duty, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode, driver::TxDatagram& tx) const override {
    if (sent == 0) return driver->gain_stm_normal_phase(gains, sent++, freq_div, mode, tx);

    switch (mode) {
      case driver::GainSTMMode::PhaseDutyFull:
        if (next_duty)
          driver->gain_stm_normal_duty(gains, sent++, freq_div, mode, tx);
        else
          driver->gain_stm_normal_phase(gains, sent, freq_div, mode, tx);
        next_duty = !next_duty;
        return;
      case driver::GainSTMMode::PhaseFull:
        return driver->gain_stm_normal_phase(gains, sent++, freq_div, mode, tx);
      default:
        throw std::runtime_error("This mode is not supported");
    }
  }

 public:
  ~NormalMode() override = default;
  static std::unique_ptr<NormalMode> create() { return std::make_unique<NormalMode>(); }
};

class NormalPhaseMode : public Mode {
  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override {
    driver->normal_header(tx);
  }
  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override {
    driver->normal_phase_body(drives, tx);
    phase_sent = true;
    duty_sent = true;
  }

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override {
    driver->gain_stm_normal_header(tx);
  }

  void pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool&, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode, driver::TxDatagram& tx) const override {
    return driver->gain_stm_normal_phase(gains, sent++, freq_div, driver::GainSTMMode::PhaseFull, tx);
  }

 public:
  ~NormalPhaseMode() override = default;
  static std::unique_ptr<NormalPhaseMode> create() { return std::make_unique<NormalPhaseMode>(); }
};

}  // namespace autd3::core
