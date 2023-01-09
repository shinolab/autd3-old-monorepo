// File: silencer_config.hpp
// Project: autd3
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/silencer.hpp"

namespace autd3::core {

/**
 * @brief DatagramHeader for silencer configuration
 */
struct SilencerConfig final : DatagramHeader {
  SilencerConfig() noexcept : SilencerConfig(10, 4096) {}
  explicit SilencerConfig(const uint16_t step, const uint16_t cycle) noexcept {
    _op.step = step;
    _op.cycle = cycle;
  }

  /**
   * @brief Create SilencerConfig to disable Silencer
   */
  static SilencerConfig none() noexcept { return SilencerConfig(0xFFFF, 4096); }

  /**
   * @brief Silencer update step.
   * @details The smaller the step, the stronger the effect of noise reduction.
   */
  [[nodiscard]] uint16_t step() const { return _op.step; }

  /**
   * @brief Silencer sampling frequency division ratio.
   * @details The sampling frequency will be driver::FPGA_CLK_FREQ/cycle. The larger the cycle, the stronger the effect of noise reduction.
   */
  [[nodiscard]] uint16_t cycle() const { return _op.step; }

  void init() override { _op.init(); }

  void pack(driver::TxDatagram& tx) override { _op.pack(tx); }

  [[nodiscard]] bool is_finished() const override { return _op.is_finished(); }

 private:
  driver::ConfigSilencer _op;
};

}  // namespace autd3::core
