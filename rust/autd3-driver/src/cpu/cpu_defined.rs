/*
 * File: cpu_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

pub const MSG_CLEAR: u8 = 0x00;
pub const MSG_RD_CPU_VERSION: u8 = 0x01;
pub const MSG_RD_FPGA_VERSION: u8 = 0x03;
pub const MSG_RD_FPGA_FUNCTION: u8 = 0x04;
pub const MSG_BEGIN: u8 = 0x05;
pub const MSG_END: u8 = 0xF0;
pub const MSG_SERVER_CLOSE: u8 = 0xFD;
pub const MSG_SIMULATOR_CLOSE: u8 = 0xFE;
pub const MSG_SIMULATOR_INIT: u8 = 0xFF;

pub const MOD_HEAD_DATA_SIZE: usize = 120;
pub const MOD_BODY_DATA_SIZE: usize = 124;

pub const POINT_STM_HEAD_DATA_SIZE: usize = 61;
pub const POINT_STM_BODY_DATA_SIZE: usize = 62;

bitflags::bitflags! {
    pub struct CPUControlFlags : u8 {
        const NONE            = 0;
        const MOD             = 1 << 0;
        const MOD_BEGIN       = 1 << 1;
        const MOD_END         = 1 << 2;
        const CONFIG_EN_N     = 1 << 0;
        const CONFIG_SILENCER = 1 << 1;
        const CONFIG_SYNC     = 1 << 2;
        const WRITE_BODY      = 1 << 3;
        const STM_BEGIN       = 1 << 4;
        const STM_END         = 1 << 5;
        const IS_DUTY         = 1 << 6;
        const MOD_DELAY       = 1 << 7;
    }
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    PhaseDutyFull = 0x0001,
    PhaseFull = 0x0002,
    PhaseHalf = 0x0004,
}
