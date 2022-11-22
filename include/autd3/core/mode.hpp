// File: mode.hpp
// Project: core
// Created Date: 28/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/11/2022
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
  [[nodiscard]] virtual bool pack_sync(const std::unique_ptr<const driver::Driver>& driver, const std::vector<uint16_t>& cycles,
                                       driver::TxDatagram& tx) const = 0;
  virtual void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const = 0;
  virtual void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                              const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const = 0;
  virtual void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const = 0;
  [[nodiscard]] virtual bool pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool& next_duty, uint32_t freq_div,
                                                const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode,
                                                driver::TxDatagram& tx) const = 0;
  virtual ~Mode() = default;
};

class LegacyMode : public Mode {
  [[nodiscard]] bool pack_sync(const std::unique_ptr<const driver::Driver>& driver, const std::vector<uint16_t>& cycles,
                               driver::TxDatagram& tx) const override;

  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override;

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  bool pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool&, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode, driver::TxDatagram& tx) const override;

 public:
  ~LegacyMode() override = default;
  static std::unique_ptr<LegacyMode> create() noexcept;
};

class NormalMode : public Mode {
  bool pack_sync(const std::unique_ptr<const driver::Driver>& driver, const std::vector<uint16_t>& cycles, driver::TxDatagram& tx) const override;

  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override;

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  bool pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool& next_duty, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode, driver::TxDatagram& tx) const override;

 public:
  ~NormalMode() override = default;
  static std::unique_ptr<NormalMode> create() noexcept;
};

class NormalPhaseMode : public Mode {
  bool pack_sync(const std::unique_ptr<const driver::Driver>& driver, const std::vector<uint16_t>& cycles, driver::TxDatagram& tx) const override;

  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override;

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  bool pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool&, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode, driver::TxDatagram& tx) const override;

 public:
  ~NormalPhaseMode() override = default;
  static std::unique_ptr<NormalPhaseMode> create() noexcept;
};

inline std::unique_ptr<Mode> legacy_mode() noexcept { return LegacyMode::create(); }
inline std::unique_ptr<Mode> normal_mode() noexcept { return NormalMode::create(); }
inline std::unique_ptr<Mode> normal_phase_mode() noexcept { return NormalPhaseMode::create(); }

}  // namespace autd3::core
