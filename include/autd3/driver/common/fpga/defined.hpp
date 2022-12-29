// File: defined.hpp
// Project: fpga
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <string>

#include "autd3/driver/defined.hpp"
#include "autd3/driver/utils.hpp"

namespace autd3::driver {

/**
 * @brief FPGA main clock frequency is 163.84MHz
 */
constexpr size_t FPGA_CLK_FREQ = 163840000;

/**
 * @brief The unit of the fixed-point number for FocusSTM is 0.025mm
 */
#ifdef AUTD3_USE_METER
constexpr autd3_float_t FOCUS_STM_FIXED_NUM_UNIT = static_cast<autd3_float_t>(0.025e-3);
#else
constexpr autd3_float_t FOCUS_STM_FIXED_NUM_UNIT = static_cast<autd3_float_t>(0.025);
#endif

/**
 * @brief Drive is a utility structure for storing ultrasound amplitude and phase.
 */
struct Drive {
  /**
   * @brief The unit of phase is radian (from 0 to 2pi)
   */
  autd3_float_t phase;
  /**
   * @brief Normalized amplitude (from 0 to 1)
   */
  autd3_float_t amp;
};

/**
 * @brief LegacyDrive stores the duty ratio/phase data actually sent to the device in Legacy mode.
 */
struct LegacyDrive {
  /**
   * @brief phase
   * @details phase=0 means 0 radian, and phase=255 means 2pi*255/256 radian
   */
  uint8_t phase;
  /**
   * @brief duty ratio of PWM signal
   */
  uint8_t duty;

  static uint8_t to_phase(const Drive d) { return static_cast<uint8_t>(static_cast<int32_t>(std::round(d.phase / (2 * pi) * 256)) & 0xFF); }

  static uint8_t to_duty(const Drive d) { return static_cast<uint8_t>(std::round(510 * std::asin(std::clamp<autd3_float_t>(d.amp, 0, 1)) / pi)); }

  void set(const Drive d) {
    phase = to_phase(d);
    duty = to_duty(d);
  }
};

/**
 * @brief Phase stores the phase data actually sent to the device in Normal/NormalPhase mode.
 */
struct Phase {
  uint16_t phase;

  static uint16_t to_phase(const Drive d, const uint16_t cycle) {
    return static_cast<uint16_t>(
        rem_euclid(static_cast<int32_t>(std::round(d.phase / (2 * pi) * static_cast<autd3_float_t>(cycle))), static_cast<int32_t>(cycle)));
  }

  void set(const Drive d, const uint16_t cycle) { phase = to_phase(d, cycle); }
};

/**
 * @brief Duty stores the duty ratio data actually sent to the device in Normal mode.
 */
struct Duty {
  uint16_t duty;

  static uint16_t to_duty(const Drive d, const uint16_t cycle) {
    return static_cast<uint16_t>(std::round(static_cast<autd3_float_t>(cycle) * std::asin(std::clamp<autd3_float_t>(d.amp, 0, 1)) / pi));
  }

  void set(const Drive d, const uint16_t cycle) { duty = to_duty(d, cycle); }
};

/**
 * @brief FPGAInfo is the state of the FPGA
 * @details Currently, it is only possible to check if the temperature of the device is above a certain level.
 */
struct FPGAInfo {
  uint8_t info;

  FPGAInfo() noexcept : info(0) {}
  explicit FPGAInfo(const uint8_t ack) noexcept : info(ack) {}

  [[nodiscard]] bool is_thermal_assert() const noexcept { return (info & 0x01) != 0; }

  [[nodiscard]] std::string to_string() const { return "Thermal assert = " + std::to_string(is_thermal_assert()); }
};

inline std::ostream& operator<<(std::ostream& os, const FPGAInfo& obj) {
  os << obj.to_string();
  return os;
}

}  // namespace autd3::driver
