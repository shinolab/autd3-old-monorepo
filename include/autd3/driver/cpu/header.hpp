// File: header.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/driver/cpu/cpu_flag.hpp"
#include "autd3/driver/fpga/fpga_flag.hpp"

namespace autd3::driver {

constexpr size_t MOD_HEADER_INITIAL_DATA_SIZE = 120;
constexpr size_t MOD_HEADER_SUBSEQUENT_DATA_SIZE = 124;

/**
 * \brief Initial Header data for Modulation
 * \details The sampling frequency division data is stored in the first 32 bits, followed by the modulation data.
 */
struct ModHeaderInitial {
  uint32_t freq_div;
  uint8_t data[MOD_HEADER_INITIAL_DATA_SIZE];
};

/**
 * \brief Subsequent Header data for Modulation
 * \details The modulation data is stored.
 */
struct ModHeaderSubsequent {
  uint8_t data[MOD_HEADER_SUBSEQUENT_DATA_SIZE];
};

/**
 * \brief Header data for Silencer
 * \details The cycle is stored in the first 16 bits, the step data in the next 16 bits, and the other is unused.
 */
struct SilencerHeader {
  uint16_t cycle;
  uint16_t step;
  uint8_t unused[120];
};

/**
 * \brief Message ID for clear operation
 */
constexpr uint8_t MSG_CLEAR = 0x00;
/**
 * \brief Message ID for read CPU firmware version operation
 */
constexpr uint8_t MSG_RD_CPU_VERSION = 0x01;
/**
 * \brief Message ID for read FPGA firmware version operation
 */
constexpr uint8_t MSG_RD_FPGA_VERSION = 0x03;
/**
 * \brief Message ID for read FPGA function operation
 */
constexpr uint8_t MSG_RD_FPGA_FUNCTION = 0x04;
/**
 * \brief Beginning of Message ID in the other operation. IDs from here to MSG_END can be used.
 */
constexpr uint8_t MSG_BEGIN = 0x05;
/**
 * \brief End of Message ID in the other operation. IDs from MSG_BEGIN to here can be used. Data with IDs after MSG_END are ignored by the actual
 * device.
 */
constexpr uint8_t MSG_END = 0xF0;
/**
 * \brief Message ID for closing remote server
 */
constexpr uint8_t MSG_SERVER_CLOSE = 0xFD;
/**
 * \brief Message ID for closing simulator
 */
constexpr uint8_t MSG_SIMULATOR_CLOSE = 0xFE;
/**
 * \brief Message ID for initializing simulator
 */
constexpr uint8_t MSG_SIMULATOR_INIT = 0xFF;

/**
 * \brief Header data for all devices
 */
struct GlobalHeader {
  /**
   * \brief Message ID
   * \details EtherCAT may send the same packet many times. Therefore, packets are distinguished by this ID and the same packets are ignored.
   * Also, some IDs are reserved for idempotent operations.
   */
  uint8_t msg_id;
  FPGAControlFlags fpga_flag;
  CPUControlFlags cpu_flag;
  /**
   * @brief Effective size of the following data
   */
  uint8_t size;
  /**
   * @brief Data area for ModHeaderInitial, ModHeaderSubsequent or SilencerHeader.
   * @details Do not use directly.
   */
  uint8_t data[124];

  GlobalHeader() noexcept : msg_id(0), fpga_flag(FPGAControlFlags::None), cpu_flag(CPUControlFlags::None), size(0), data() {}

  [[nodiscard]] const ModHeaderInitial& mod_initial() const noexcept { return *reinterpret_cast<ModHeaderInitial const*>(data); }
  ModHeaderInitial& mod_initial() noexcept { return *reinterpret_cast<ModHeaderInitial*>(data); }

  [[nodiscard]] const ModHeaderSubsequent& mod_subsequent() const noexcept { return *reinterpret_cast<ModHeaderSubsequent const*>(data); }
  ModHeaderSubsequent& mod_subsequent() noexcept { return *reinterpret_cast<ModHeaderSubsequent*>(data); }

  [[nodiscard]] const SilencerHeader& silencer() const noexcept { return *reinterpret_cast<SilencerHeader const*>(data); }
  SilencerHeader& silencer() noexcept { return *reinterpret_cast<SilencerHeader*>(data); }
};

}  // namespace autd3::driver
