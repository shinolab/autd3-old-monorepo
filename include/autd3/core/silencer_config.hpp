// File: silencer_config.hpp
// Project: autd3
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/core/interface.hpp"

namespace autd3::core {

/**
 * @brief Silencer Configuration
 */
struct SilencerConfig final : DatagramHeader {
  SilencerConfig() noexcept : SilencerConfig(10, 4096) {}
  explicit SilencerConfig(const uint16_t step, const uint16_t cycle) noexcept : step(step), cycle(cycle), _sent(false) {}

  /**
   * @brief Create SilencerConfig to disable Silencer
   */
  static SilencerConfig none() noexcept { return SilencerConfig(0xFFFF, 4096); }

  /**
   * @brief Silencer update step.
   * @details The smaller the step, the stronger the effect of noise reduction.
   */
  uint16_t step;
  /**
   * @brief Silencer sampling frequency division ratio.
   * @details The sampling frequency will be driver::FPGA_CLK_FREQ/cycle. The larger the cycle, the stronger the effect of noise reduction.
   */
  uint16_t cycle;

  bool init() override {
    _sent = false;
    return true;
  }

  bool pack(const std::unique_ptr<const driver::Driver>& driver, const uint8_t msg_id, driver::TxDatagram& tx) override {
    if (_sent) {
      driver->null_header(msg_id, tx);
      return true;
    }

    _sent = true;
    return driver->config_silencer(msg_id, cycle, step, tx);
  }

  [[nodiscard]] bool is_finished() const override { return true; }

 private:
  bool _sent;
};

}  // namespace autd3::core
