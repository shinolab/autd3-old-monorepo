// File: silencer_config.hpp
// Project: autd3
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>
#include <memory>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/silencer.hpp"

namespace autd3::core {

/**
 * @brief DatagramHeader for silencer configuration
 */
struct SilencerConfig final : DatagramHeader {
  SilencerConfig() noexcept : SilencerConfig(10, 4096) {}
  explicit SilencerConfig(const uint16_t step, const uint16_t cycle) noexcept {
    _step = step;
    _cycle = cycle;
  }

  /**
   * @brief Create SilencerConfig to disable Silencer
   */
  static SilencerConfig none() noexcept { return SilencerConfig(0xFFFF, 4096); }

  /**
   * @brief Silencer update step.
   * @details The smaller the step, the stronger the effect of noise reduction.
   */
  [[nodiscard]] uint16_t step() const { return _step; }

  /**
   * @brief Silencer sampling frequency division ratio.
   * @details The sampling frequency will be driver::FPGA_CLK_FREQ/cycle. The larger the cycle, the stronger the effect of noise reduction.
   */
  [[nodiscard]] uint16_t cycle() const { return _cycle; }

  std::unique_ptr<driver::Operation> operation() override { return std::make_unique<driver::ConfigSilencer>(_cycle, _step); }

 private:
  uint16_t _step;
  uint16_t _cycle;
};

}  // namespace autd3::core
