// File: mode.hpp
// Project: core
// Created Date: 28/06/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <vector>

#include "autd3/driver/driver.hpp"

namespace autd3::core {

/**
 * @brief Amplitude, phase, and frequency control mode
 */
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
  Mode() = default;
  virtual ~Mode() = default;
  Mode(const Mode& v) = default;
  Mode& operator=(const Mode& obj) = default;
  Mode(Mode&& obj) = default;
  Mode& operator=(Mode&& obj) = default;
};

/**
 * @brief In LegacyMode, the frequency is fixed at 40 kHz, and the width of phase and amplitude data is 8 bits, respectively.
 */
class LegacyMode final : public Mode {
  [[nodiscard]] bool pack_sync(const std::unique_ptr<const driver::Driver>& driver, const std::vector<uint16_t>& cycles,
                               driver::TxDatagram& tx) const override;

  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override;

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  bool pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool&, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode, driver::TxDatagram& tx) const override;

 public:
  LegacyMode() = default;
  ~LegacyMode() override = default;
  LegacyMode(const LegacyMode& v) = default;
  LegacyMode& operator=(const LegacyMode& obj) = default;
  LegacyMode(LegacyMode&& obj) = default;
  LegacyMode& operator=(LegacyMode&& obj) = default;
  static std::unique_ptr<LegacyMode> create() noexcept;
};

/**
 * @brief In NormalMode, the frequency is variable. Amplitude and phase data can be controlled with a fineness of driver::FPGA_CLK_FREQ/frequency.
 */
class NormalMode final : public Mode {
  bool pack_sync(const std::unique_ptr<const driver::Driver>& driver, const std::vector<uint16_t>& cycles, driver::TxDatagram& tx) const override;

  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override;

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  bool pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool& next_duty, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode mode, driver::TxDatagram& tx) const override;

 public:
  NormalMode() = default;
  ~NormalMode() override = default;
  NormalMode(const NormalMode& v) = default;
  NormalMode& operator=(const NormalMode& obj) = default;
  NormalMode(NormalMode&& obj) = default;
  NormalMode& operator=(NormalMode&& obj) = default;
  static std::unique_ptr<NormalMode> create() noexcept;
};

/**
 * @brief NormalPhaseMode is equivalent to NormalMode, except for it transmits only phase data.
 */
class NormalPhaseMode final : public Mode {
  bool pack_sync(const std::unique_ptr<const driver::Driver>& driver, const std::vector<uint16_t>& cycles, driver::TxDatagram& tx) const override;

  void pack_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  void pack_gain_body(const std::unique_ptr<const driver::Driver>& driver, bool& phase_sent, bool& duty_sent,
                      const std::vector<driver::Drive>& drives, driver::TxDatagram& tx) const override;

  void pack_stm_gain_header(const std::unique_ptr<const driver::Driver>& driver, driver::TxDatagram& tx) const noexcept override;

  bool pack_stm_gain_body(const std::unique_ptr<const driver::Driver>& driver, size_t& sent, bool&, uint32_t freq_div,
                          const std::vector<std::vector<driver::Drive>>& gains, driver::GainSTMMode, driver::TxDatagram& tx) const override;

 public:
  NormalPhaseMode() = default;
  ~NormalPhaseMode() override = default;
  NormalPhaseMode(const NormalPhaseMode& v) = default;
  NormalPhaseMode& operator=(const NormalPhaseMode& obj) = default;
  NormalPhaseMode(NormalPhaseMode&& obj) = default;
  NormalPhaseMode& operator=(NormalPhaseMode&& obj) = default;
  static std::unique_ptr<NormalPhaseMode> create() noexcept;
};

inline std::unique_ptr<Mode> legacy_mode() noexcept { return LegacyMode::create(); }
inline std::unique_ptr<Mode> normal_mode() noexcept { return NormalMode::create(); }
inline std::unique_ptr<Mode> normal_phase_mode() noexcept { return NormalPhaseMode::create(); }

}  // namespace autd3::core
