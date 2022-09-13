// File: params.hpp
// Project: cpu
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::extra::firmware_emulator::cpu {
constexpr uint16_t CPU_VERSION = 0x83;

constexpr uint8_t BRAM_SELECT_CONTROLLER = 0x0;
constexpr uint8_t BRAM_SELECT_MOD = 0x1;
constexpr uint8_t BRAM_SELECT_NORMAL = 0x2;
constexpr uint8_t BRAM_SELECT_STM = 0x3;

constexpr uint16_t BRAM_ADDR_CTL_REG = 0x000;
constexpr uint16_t BRAM_ADDR_FPGA_INFO = 0x001;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_0 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 1;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_1 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 2;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_2 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 3;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_3 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 4;
constexpr uint16_t BRAM_ADDR_MOD_ADDR_OFFSET = 0x020;
constexpr uint16_t BRAM_ADDR_MOD_CYCLE = 0x021;
constexpr uint16_t BRAM_ADDR_MOD_FREQ_DIV_0 = 0x022;
constexpr uint16_t BRAM_ADDR_VERSION_NUM = 0x03F;
constexpr uint16_t BRAM_ADDR_SILENT_CYCLE = 0x040;
constexpr uint16_t BRAM_ADDR_SILENT_STEP = 0x041;
constexpr uint16_t BRAM_ADDR_STM_ADDR_OFFSET = 0x050;
constexpr uint16_t BRAM_ADDR_STM_CYCLE = 0x051;
constexpr uint16_t BRAM_ADDR_STM_FREQ_DIV_0 = 0x052;
constexpr uint16_t BRAM_ADDR_SOUND_SPEED_0 = 0x054;
constexpr uint16_t BRAM_ADDR_CYCLE_BASE = 0x100;
constexpr uint16_t BRAM_ADDR_MOD_DELAY_BASE = 0x200;

constexpr uint32_t MOD_BUF_SEGMENT_SIZE_WIDTH = 15;
constexpr uint32_t MOD_BUF_SEGMENT_SIZE = 1 << MOD_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t MOD_BUF_SEGMENT_SIZE_MASK = MOD_BUF_SEGMENT_SIZE - 1;
constexpr uint32_t POINT_STM_BUF_SEGMENT_SIZE_WIDTH = 11;
constexpr uint32_t POINT_STM_BUF_SEGMENT_SIZE = 1 << POINT_STM_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t POINT_STM_BUF_SEGMENT_SIZE_MASK = POINT_STM_BUF_SEGMENT_SIZE - 1;
constexpr uint32_t GAIN_STM_BUF_SEGMENT_SIZE_WIDTH = 5;
constexpr uint32_t GAIN_STM_BUF_SEGMENT_SIZE = 1 << GAIN_STM_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t GAIN_STM_BUF_SEGMENT_SIZE_MASK = GAIN_STM_BUF_SEGMENT_SIZE - 1;
constexpr uint32_t GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH = 6;
constexpr uint32_t GAIN_STM_LEGACY_BUF_SEGMENT_SIZE = 1 << GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK = GAIN_STM_LEGACY_BUF_SEGMENT_SIZE - 1;
}  // namespace autd3::extra::firmware_emulator::cpu
