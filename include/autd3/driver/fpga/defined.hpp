// File: defined.hpp
// Project: fpga
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/01/2023
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
 * @brief Ultrasound amplitude
 * @details The value is normalized from 0 to 1
 */
struct Amp {
  explicit Amp(const autd3_float_t amp) : _value(std::clamp<autd3_float_t>(amp, 0, 1)) {}

  Amp() = default;
  Amp(const Amp& v) = default;
  Amp& operator=(const Amp& obj) = default;
  Amp(Amp&& obj) = default;
  Amp& operator=(Amp&& obj) = default;
  ~Amp() = default;

  [[nodiscard]] autd3_float_t value() const { return _value; }

 private:
  autd3_float_t _value;
};

/**
 * @brief Ultrasound phase
 * @details The unit is radian
 */
struct Phase {
  explicit Phase(const autd3_float_t phase) : _value(phase) {}

  Phase() = default;
  Phase(const Phase& v) = default;
  Phase& operator=(const Phase& obj) = default;
  Phase(Phase&& obj) = default;
  Phase& operator=(Phase&& obj) = default;
  ~Phase() = default;

  [[nodiscard]] autd3_float_t value() const { return _value; }

 private:
  autd3_float_t _value;
};

/**
 * @brief Drive is a utility structure for storing ultrasound amplitude and phase.
 */
struct Drive {
  /**
   * @brief The unit of phase is radian (from 0 to 2pi)
   */
  Phase phase{0};
  /**
   * @brief Normalized amplitude (from 0 to 1)
   */
  Amp amp{0};
};

/**
 * @brief LegacyDrive stores the duty ratio/phase data actually sent to the device in Legacy mode.
 */
#pragma pack(push)
#pragma pack(1)
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

  static uint8_t to_phase(const Drive d) { return static_cast<uint8_t>(static_cast<int32_t>(std::round(d.phase.value() / (2 * pi) * 256)) & 0xFF); }

  static uint8_t to_duty(const Drive d) { return static_cast<uint8_t>(std::round(510 * std::asin(d.amp.value()) / pi)); }

  void set(const Drive d) {
    phase = to_phase(d);
    duty = to_duty(d);
  }

  LegacyDrive& operator=(const Drive& d) {
    set(d);
    return *this;
  }
  LegacyDrive& operator=(Drive&& d) {
    set(d);
    return *this;
  }
};
#pragma pack(pop)

/**
 * @brief Phase stores the phase data actually sent to the device in Normal/NormalPhase mode.
 */
#pragma pack(push)
#pragma pack(1)
struct NormalDrivePhase {
  uint16_t phase;

  static uint16_t to_phase(const Drive d, const uint16_t cycle) {
    return static_cast<uint16_t>(
        rem_euclid(static_cast<int32_t>(std::round(d.phase.value() / (2 * pi) * static_cast<autd3_float_t>(cycle))), static_cast<int32_t>(cycle)));
  }

  void set(const Drive d, const uint16_t cycle) { phase = to_phase(d, cycle); }
  explicit NormalDrivePhase(const Drive d, const uint16_t cycle) : phase(to_phase(d, cycle)) {}
  explicit NormalDrivePhase(const uint16_t phase) : phase(phase) {}
  NormalDrivePhase(const NormalDrivePhase& v) = default;
  NormalDrivePhase& operator=(const NormalDrivePhase& obj) = default;
  NormalDrivePhase(NormalDrivePhase&& obj) = default;
  NormalDrivePhase& operator=(NormalDrivePhase&& obj) = default;
  ~NormalDrivePhase() = default;
};
#pragma pack(pop)

/**
 * @brief Duty stores the duty ratio data actually sent to the device in Normal mode.
 */
#pragma pack(push)
#pragma pack(1)
struct NormalDriveDuty {
  uint16_t duty;

  static uint16_t to_duty(const Drive d, const uint16_t cycle) {
    return static_cast<uint16_t>(std::round(static_cast<autd3_float_t>(cycle) * std::asin(d.amp.value()) / pi));
  }

  void set(const Drive d, const uint16_t cycle) { duty = to_duty(d, cycle); }
  explicit NormalDriveDuty(const Drive d, const uint16_t cycle) : duty(to_duty(d, cycle)) {}
  explicit NormalDriveDuty(const uint16_t duty) : duty(duty) {}
  NormalDriveDuty(const NormalDriveDuty& v) = default;
  NormalDriveDuty& operator=(const NormalDriveDuty& obj) = default;
  NormalDriveDuty(NormalDriveDuty&& obj) = default;
  NormalDriveDuty& operator=(NormalDriveDuty&& obj) = default;
  ~NormalDriveDuty() = default;
};
#pragma pack(pop)

/**
 * @brief FPGAInfo is the state of the FPGA
 * @details Currently, it is only possible to check if the temperature of the device is above a certain level.
 */
#pragma pack(push)
#pragma pack(1)
struct FPGAInfo {
  uint8_t info;

  FPGAInfo() noexcept : info(0) {}
  explicit FPGAInfo(const uint8_t ack) noexcept : info(ack) {}

  [[nodiscard]] bool is_thermal_assert() const noexcept { return (info & 0x01) != 0; }

  [[nodiscard]] std::string to_string() const { return "Thermal assert = " + std::to_string(is_thermal_assert()); }
};
#pragma pack(pop)

inline std::ostream& operator<<(std::ostream& os, const FPGAInfo& obj) {
  os << obj.to_string();
  return os;
}

}  // namespace autd3::driver
