/*
 * File: params.rs
 * Project: fpga
 * Created Date: 07/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::defined::{float, METER};

pub const VERSION_NUM_MAJOR: u8 = 0x8A;
pub const VERSION_NUM_MINOR: u8 = 0x00;

pub const TRANS_SIZE_FIXED_POINT_UNIT: float = 40000. / METER;

pub const BRAM_SELECT_CONTROLLER: u16 = 0x0;
pub const BRAM_SELECT_MOD: u16 = 0x1;
pub const BRAM_SELECT_NORMAL: u16 = 0x2;
pub const BRAM_SELECT_STM: u16 = 0x3;

// pub const BRAM_SELECT_CONTROLLER_MAIN: u16 = 0b000;
// pub const BRAM_SELECT_CONTROLLER_CYCLE: u16 = 0b001;
// pub const BRAM_SELECT_CONTROLLER_DELAY: u16 = 0b010;
// pub const BRAM_SELECT_CONTROLLER_FILTER_DUTY: u16 = 0b011;
// pub const BRAM_SELECT_CONTROLLER_FILTER_PHASE: u16 = 0b100;

pub const ADDR_CTL_REG: usize = 0x0000;
pub const ADDR_FPGA_INFO: usize = 0x0001;
pub const ADDR_MOD_ADDR_OFFSET: usize = 0x0020;
pub const ADDR_MOD_CYCLE: usize = 0x0021;
pub const ADDR_MOD_FREQ_DIV_0: usize = 0x0022;
pub const ADDR_MOD_FREQ_DIV_1: usize = 0x0023;
pub const ADDR_VERSION_NUM: usize = 0x0030;
pub const ADDR_VERSION_NUM_MINOR: usize = 0x0031;
pub const ADDR_SILENT_STEP: usize = 0x0040;
pub const ADDR_STM_ADDR_OFFSET: usize = 0x0050;
pub const ADDR_STM_CYCLE: usize = 0x0051;
pub const ADDR_STM_FREQ_DIV_0: usize = 0x0052;
pub const ADDR_STM_FREQ_DIV_1: usize = 0x0053;
pub const ADDR_SOUND_SPEED_0: usize = 0x0054;
pub const ADDR_SOUND_SPEED_1: usize = 0x0055;
pub const ADDR_STM_START_IDX: usize = 0x0056;
pub const ADDR_STM_FINISH_IDX: usize = 0x0057;
pub const ADDR_CYCLE_BASE: usize = 0x0100;
pub const ADDR_MOD_DELAY_BASE: usize = 0x0200;
pub const ADDR_FILTER_DUTY_BASE: usize = 0x0300;
pub const ADDR_FILTER_PHASE_BASE: usize = 0x0400;

pub const CTL_REG_FORCE_FAN_BIT: u16 = 0;
pub const CTL_REG_LEGACY_MODE_BIT: u16 = 8;
pub const CTL_REG_OP_MODE_BIT: u16 = 9;
pub const CTL_REG_STM_GAIN_MODE_BIT: u16 = 10;
pub const CTL_FLAG_USE_STM_FINISH_IDX_BIT: u16 = 11;
pub const CTL_FLAG_USE_STM_START_IDX_BIT: u16 = 12;

pub const ENABLED_STM_BIT: u8 = 0x01;
pub const ENABLED_MODULATOR_BIT: u8 = 0x02;
pub const ENABLED_SILENCER_BIT: u8 = 0x04;
pub const ENABLED_MOD_DELAY_BIT: u8 = 0x08;
pub const ENABLED_FILTER_BIT: u8 = 0x10;
pub const ENABLED_EMULATOR_BIT: u8 = 0x80;
pub const ENABLED_FEATURES_BITS: u8 = ENABLED_MOD_DELAY_BIT
    | ENABLED_STM_BIT
    | ENABLED_MODULATOR_BIT
    | ENABLED_SILENCER_BIT
    | ENABLED_FILTER_BIT
    | ENABLED_EMULATOR_BIT;
