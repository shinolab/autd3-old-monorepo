// File: params.hpp
// Project: fpga
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::extra::firmware_emulator::fpga {

constexpr uint8_t VERSION_NUM = 0x84;

constexpr uint16_t BRAM_SELECT_CONTROLLER = 0x0;
constexpr uint16_t BRAM_SELECT_MOD = 0x1;
constexpr uint16_t BRAM_SELECT_NORMAL = 0x2;
constexpr uint16_t BRAM_SELECT_STM = 0x3;

constexpr size_t ADDR_CTL_REG = 0x0000;
// constexpr size_t ADDR_FPGA_INFO = 0x0001;
// constexpr size_t ADDR_EC_SYNC_TIME_0 = ADDR_EC_SYNC_CYCLE_TICKS + 1;
// constexpr size_t ADDR_EC_SYNC_TIME_1 = ADDR_EC_SYNC_CYCLE_TICKS + 2;
// constexpr size_t ADDR_EC_SYNC_TIME_2 = ADDR_EC_SYNC_CYCLE_TICKS + 3;
// constexpr size_t ADDR_EC_SYNC_TIME_3 = ADDR_EC_SYNC_CYCLE_TICKS + 4;
constexpr size_t ADDR_MOD_ADDR_OFFSET = 0x0020;
constexpr size_t ADDR_MOD_CYCLE = 0x0021;
constexpr size_t ADDR_MOD_FREQ_DIV_0 = 0x0022;
constexpr size_t ADDR_MOD_FREQ_DIV_1 = 0x0023;
constexpr size_t ADDR_VERSION_NUM = 0x003F;
constexpr size_t ADDR_SILENT_CYCLE = 0x0040;
constexpr size_t ADDR_SILENT_STEP = 0x0041;
constexpr size_t ADDR_STM_ADDR_OFFSET = 0x0050;
constexpr size_t ADDR_STM_CYCLE = 0x0051;
constexpr size_t ADDR_STM_FREQ_DIV_0 = 0x0052;
constexpr size_t ADDR_STM_FREQ_DIV_1 = 0x0053;
constexpr size_t ADDR_SOUND_SPEED_0 = 0x0054;
constexpr size_t ADDR_SOUND_SPEED_1 = 0x0055;
constexpr size_t ADDR_CYCLE_BASE = 0x0100;
constexpr size_t ADDR_MOD_DELAY_BASE = 0x0200;

constexpr uint16_t CTL_REG_LEGACY_MODE_BIT = 0;
constexpr uint16_t CTL_REG_FORCE_FAN_BIT = 4;
constexpr uint16_t CTL_REG_OP_MODE_BIT = 5;
constexpr uint16_t CTL_REG_STM_GAIN_MODE_BIT = 6;
// constexpr size_t CTL_REG_SYNC_BIT = 8;

constexpr uint8_t ENABLED_STM_BIT = 0x01;
constexpr uint8_t ENABLED_MODULATOR_BIT = 0x02;
constexpr uint8_t ENABLED_SILENCER_BIT = 0x04;
constexpr uint8_t ENABLED_MOD_DELAY_BIT = 0x08;
constexpr uint8_t ENABLED_FEATURES_BITS = ENABLED_MOD_DELAY_BIT | ENABLED_STM_BIT | ENABLED_MODULATOR_BIT | ENABLED_SILENCER_BIT;

}  // namespace autd3::extra::firmware_emulator::fpga
