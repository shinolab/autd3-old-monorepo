/*
 * File: params.rs
 * Project: cpu
 * Created Date: 07/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

pub const CPU_VERSION_MAJOR: u16 = 0x8A;
pub const CPU_VERSION_MINOR: u16 = 0x00;

pub const BRAM_SELECT_CONTROLLER: u8 = 0x0;
pub const BRAM_SELECT_MOD: u8 = 0x1;
pub const BRAM_SELECT_NORMAL: u8 = 0x2;
pub const BRAM_SELECT_STM: u8 = 0x3;

pub const CTL_REG_LEGACY_MODE: u16 = 1 << 8;
pub const CTL_FLAG_OP_MODE: u16 = 1 << 9;
pub const CTL_REG_STM_GAIN_MODE: u16 = 1 << 10;
pub const CTL_FLAG_USE_STM_FINISH_IDX: u16 = 1 << 11;
pub const CTL_FLAG_USE_STM_START_IDX: u16 = 1 << 12;

pub const BRAM_ADDR_CTL_REG: u16 = 0x000;
pub const BRAM_ADDR_FPGA_INFO: u16 = 0x001;
pub const BRAM_ADDR_MOD_ADDR_OFFSET: u16 = 0x020;
pub const BRAM_ADDR_MOD_CYCLE: u16 = 0x021;
pub const BRAM_ADDR_MOD_FREQ_DIV_0: u16 = 0x022;
pub const BRAM_ADDR_VERSION_NUM: u16 = 0x030;
pub const BRAM_ADDR_VERSION_NUM_MINOR: u16 = 0x031;
pub const BRAM_ADDR_SILENT_STEP: u16 = 0x040;
pub const BRAM_ADDR_STM_ADDR_OFFSET: u16 = 0x050;
pub const BRAM_ADDR_STM_CYCLE: u16 = 0x051;
pub const BRAM_ADDR_STM_FREQ_DIV_0: u16 = 0x052;
pub const BRAM_ADDR_SOUND_SPEED_0: u16 = 0x054;
pub const BRAM_ADDR_STM_START_IDX: u16 = 0x056;
pub const BRAM_ADDR_STM_FINISH_IDX: u16 = 0x057;
pub const BRAM_ADDR_CYCLE_BASE: u16 = 0x100;
pub const BRAM_ADDR_MOD_DELAY_BASE: u16 = 0x200;
pub const BRAM_ADDR_FILTER_DUTY_BASE: u16 = 0x300;
pub const BRAM_ADDR_FILTER_PHASE_BASE: u16 = 0x400;

pub const MOD_BUF_SEGMENT_SIZE_WIDTH: u32 = 15;
pub const MOD_BUF_SEGMENT_SIZE: u32 = 1 << MOD_BUF_SEGMENT_SIZE_WIDTH;
pub const MOD_BUF_SEGMENT_SIZE_MASK: u32 = MOD_BUF_SEGMENT_SIZE - 1;
pub const POINT_STM_BUF_SEGMENT_SIZE_WIDTH: u32 = 11;
pub const POINT_STM_BUF_SEGMENT_SIZE: u32 = 1 << POINT_STM_BUF_SEGMENT_SIZE_WIDTH;
pub const POINT_STM_BUF_SEGMENT_SIZE_MASK: u32 = POINT_STM_BUF_SEGMENT_SIZE - 1;
pub const GAIN_STM_BUF_SEGMENT_SIZE_WIDTH: u32 = 5;
pub const GAIN_STM_BUF_SEGMENT_SIZE: u32 = 1 << GAIN_STM_BUF_SEGMENT_SIZE_WIDTH;
pub const GAIN_STM_BUF_SEGMENT_SIZE_MASK: u32 = GAIN_STM_BUF_SEGMENT_SIZE - 1;
pub const GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH: u32 = 6;
pub const GAIN_STM_LEGACY_BUF_SEGMENT_SIZE: u32 = 1 << GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH;
pub const GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK: u32 = GAIN_STM_LEGACY_BUF_SEGMENT_SIZE - 1;

pub const TAG_NONE: u8 = 0x00;
pub const TAG_CLEAR: u8 = 0x01;
pub const TAG_SYNC: u8 = 0x02;
pub const TAG_FIRM_INFO: u8 = 0x03;
pub const TAG_UPDATE_FLAGS: u8 = 0x04;
pub const TAG_MODULATION: u8 = 0x10;
pub const TAG_MODULATION_DELAY: u8 = 0x11;
pub const TAG_SILENCER: u8 = 0x20;
pub const TAG_GAIN: u8 = 0x30;
pub const TAG_FOCUS_STM: u8 = 0x40;
pub const TAG_GAIN_STM: u8 = 0x50;
pub const TAG_FILTER: u8 = 0x60;

pub const INFO_TYPE_CPU_VERSION_MAJOR: u8 = 0x01;
pub const INFO_TYPE_CPU_VERSION_MINOR: u8 = 0x02;
pub const INFO_TYPE_FPGA_VERSION_MAJOR: u8 = 0x03;
pub const INFO_TYPE_FPGA_VERSION_MINOR: u8 = 0x04;
pub const INFO_TYPE_FPGA_FUNCTIONS: u8 = 0x05;
pub const INFO_TYPE_CLEAR: u8 = 0x06;

pub const MODULATION_FLAG_BEGIN: u8 = 1 << 0;
pub const MODULATION_FLAG_END: u8 = 1 << 1;

pub const GAIN_FLAG_LEGACY: u8 = 1 << 0;
pub const GAIN_FLAG_DUTY: u8 = 1 << 1;

pub const FOCUS_STM_FLAG_BEGIN: u8 = 1 << 0;
pub const FOCUS_STM_FLAG_END: u8 = 1 << 1;
pub const FOCUS_STM_FLAG_USE_START_IDX: u8 = 1 << 2;
pub const FOCUS_STM_FLAG_USE_FINISH_IDX: u8 = 1 << 3;

pub const GAIN_STM_FLAG_LEGACY: u8 = 1 << 0;
pub const GAIN_STM_FLAG_DUTY: u8 = 1 << 1;
pub const GAIN_STM_FLAG_BEGIN: u8 = 1 << 2;
pub const GAIN_STM_FLAG_END: u8 = 1 << 3;
pub const GAIN_STM_FLAG_USE_START_IDX: u8 = 1 << 4;
pub const GAIN_STM_FLAG_USE_FINISH_IDX: u8 = 1 << 5;
pub const GAIN_STM_FLAG_IGNORE_DUTY: u8 = 1 << 6;
pub const GAIN_STM_FLAG_PHASE_COMPRESS: u8 = 1 << 7;

pub const FILTER_ADD_PHASE: u8 = 0x00;
pub const FILTER_ADD_DUTY: u8 = 0x01;
