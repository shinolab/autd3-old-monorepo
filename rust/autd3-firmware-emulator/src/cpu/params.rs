/*
 * File: params.rs
 * Project: cpu
 * Created Date: 07/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

pub const CPU_VERSION_MAJOR: u16 = 0x89;
pub const CPU_VERSION_MINOR: u16 = 0x00;

pub const BRAM_SELECT_CONTROLLER: u8 = 0x0;
pub const BRAM_SELECT_MOD: u8 = 0x1;
pub const BRAM_SELECT_NORMAL: u8 = 0x2;
pub const BRAM_SELECT_STM: u8 = 0x3;

pub const CTL_FLAG_OP_MODE_BIT: u16 = 9;
pub const CTL_FLAG_OP_MODE: u16 = 1 << CTL_FLAG_OP_MODE_BIT;

pub const BRAM_ADDR_CTL_REG: u16 = 0x000;
pub const BRAM_ADDR_FPGA_INFO: u16 = 0x001;
pub const BRAM_ADDR_MOD_ADDR_OFFSET: u16 = 0x020;
pub const BRAM_ADDR_MOD_CYCLE: u16 = 0x021;
pub const BRAM_ADDR_MOD_FREQ_DIV_0: u16 = 0x022;
pub const BRAM_ADDR_VERSION_NUM: u16 = 0x03F;
pub const BRAM_ADDR_VERSION_NUM_MINOR: u16 = 0x03E;
pub const BRAM_ADDR_SILENT_STEP: u16 = 0x041;
pub const BRAM_ADDR_STM_ADDR_OFFSET: u16 = 0x050;
pub const BRAM_ADDR_STM_CYCLE: u16 = 0x051;
pub const BRAM_ADDR_STM_FREQ_DIV_0: u16 = 0x052;
pub const BRAM_ADDR_SOUND_SPEED_0: u16 = 0x054;
pub const BRAM_ADDR_STM_START_IDX: u16 = 0x056;
pub const BRAM_ADDR_STM_FINISH_IDX: u16 = 0x057;
pub const BRAM_ADDR_CYCLE_BASE: u16 = 0x100;
pub const BRAM_ADDR_MOD_DELAY_BASE: u16 = 0x200;

pub const MOD_BUF_SEGMENT_SIZE_WIDTH: u32 = 15;
pub const MOD_BUF_SEGMENT_SIZE: u32 = 1 << MOD_BUF_SEGMENT_SIZE_WIDTH;
pub const MOD_BUF_SEGMENT_SIZE_MASK: u32 = MOD_BUF_SEGMENT_SIZE - 1;
pub const POINT_STM_BUF_SEGMENT_SIZE_WIDTH: u32 = 11;
pub const POINT_STM_BUF_SEGMENT_SIZE: u32 = 1 << POINT_STM_BUF_SEGMENT_SIZE_WIDTH;
pub const POINT_STM_BUF_SEGMENT_SIZE_MASK: u32 = POINT_STM_BUF_SEGMENT_SIZE - 1;
pub const GAIN_STM_BUF_SEGMENT_SIZE_WIDTH: u32 = 5;
pub const GAIN_STM_BUF_SEGMENT_SIZE: u32 = 1 << GAIN_STM_BUF_SEGMENT_SIZE_WIDTH;
pub const GAIN_STM_BUF_SEGMENT_SIZE_MASK: u32 = GAIN_STM_BUF_SEGMENT_SIZE - 1;

pub const GAIN_STM_MODE_PHASE_DUTY_FULL: u16 = 0x0001;
pub const GAIN_STM_MODE_PHASE_FULL: u16 = 0x0002;
pub const GAIN_STM_MODE_PHASE_HALF: u16 = 0x0004;
