// File: firmware_version.hpp
// Project: driver
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>
#include <sstream>
#include <string>

namespace autd3::driver {

constexpr uint8_t ENABLED_STM_BIT = 1 << 0;
constexpr uint8_t ENABLED_MODULATOR_BIT = 1 << 1;
constexpr uint8_t ENABLED_SILENCER_BIT = 1 << 2;
constexpr uint8_t ENABLED_MOD_DELAY_BIT = 1 << 3;
constexpr uint8_t ENABLED_EMULATOR_BIT = 1 << 7;

/**
 * \brief Firmware information
 */
struct FirmwareInfo {
  FirmwareInfo(const size_t idx, const uint8_t cpu_version_number, const uint8_t fpga_version_number, const uint8_t fpga_function_bits) noexcept
      : _idx(idx), _cpu_version_number(cpu_version_number), _fpga_version_number(fpga_version_number), _fpga_function_bits(fpga_function_bits) {}

  /**
   * \brief Get cpu firmware version
   */
  [[nodiscard]] std::string cpu_version() const { return firmware_version_map(_cpu_version_number); }
  /**
   * \brief Get fpga firmware version
   */
  [[nodiscard]] std::string fpga_version() const { return firmware_version_map(_fpga_version_number); }

  /**
   * \return true if the firmware supports STM function
   */
  [[nodiscard]] bool stm_enabled() const noexcept { return (_fpga_function_bits & ENABLED_STM_BIT) != 0; }
  /**
   * \return true if the firmware supports Modulator function
   */
  [[nodiscard]] bool modulator_enabled() const noexcept { return (_fpga_function_bits & ENABLED_MODULATOR_BIT) != 0; }
  /**
   * \return true if the firmware supports Silencer function
   */
  [[nodiscard]] bool silencer_enabled() const noexcept { return (_fpga_function_bits & ENABLED_SILENCER_BIT) != 0; }

  /**
   * \return true if the firmware supports Modulation delay function
   */
  [[nodiscard]] bool modulation_delay_enabled() const noexcept { return (_fpga_function_bits & ENABLED_MOD_DELAY_BIT) != 0; }

  [[nodiscard]] std::string to_string() const {
    std::stringstream ss;
    ss << _idx << ": CPU = " << cpu_version() << ", FPGA = " << fpga_version() << " (STM = " << std::boolalpha << stm_enabled()
       << ", Modulator = " << modulator_enabled() << ", Silencer = " << silencer_enabled() << ", ModDelay = " << modulation_delay_enabled() << ")";
    if (is_emulator()) ss << " [Emulator]";
    return ss.str();
  }

  [[nodiscard]] bool is_emulator() const { return (_fpga_function_bits & ENABLED_EMULATOR_BIT) != 0; }

 private:
  size_t _idx;
  uint8_t _cpu_version_number;
  uint8_t _fpga_version_number;
  uint8_t _fpga_function_bits;

  [[nodiscard]] static std::string firmware_version_map(const uint8_t version_num) {
    if (version_num == 0) return "older than v0.4";
    if (version_num <= 0x06) return "v0." + std::to_string(version_num + 3);
    if (version_num <= 0x09) return "unknown (" + std::to_string(static_cast<int>(version_num)) + ")";
    if (version_num <= 0x15) return "v1." + std::to_string(version_num - 0x0A);
    if (version_num <= 0x85) return "v2." + std::to_string(version_num - 0x80);
    return "unknown (" + std::to_string(static_cast<int>(version_num)) + ")";
  }
};

inline std::ostream& operator<<(std::ostream& os, const FirmwareInfo& obj) {
  os << obj.to_string();
  return os;
}

}  // namespace autd3::driver
