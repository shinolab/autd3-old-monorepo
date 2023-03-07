/*
 * File: header.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{cpu::CPUControlFlags, fpga::FPGAControlFlags};

pub const MSG_CLEAR: u8 = 0x00;
pub const MSG_RD_CPU_VERSION: u8 = 0x01;
pub const MSG_RD_FPGA_VERSION: u8 = 0x03;
pub const MSG_RD_FPGA_FUNCTION: u8 = 0x04;
pub const MSG_BEGIN: u8 = 0x05;
pub const MSG_END: u8 = 0xF0;
pub const MSG_RD_CPU_VERSION_MINOR: u8 = 0xF1;
pub const MSG_RD_FPGA_VERSION_MINOR: u8 = 0xF2;
pub const MSG_SERVER_CLOSE: u8 = 0xFD;
pub const MSG_SIMULATOR_CLOSE: u8 = 0xFE;
pub const MSG_SIMULATOR_INIT: u8 = 0xFF;

pub const MOD_HEADER_INITIAL_DATA_SIZE: usize = 120;
pub const MOD_HEADER_SUBSEQUENT_DATA_SIZE: usize = 124;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GlobalHeader {
    pub msg_id: u8,
    pub fpga_flag: FPGAControlFlags,
    pub cpu_flag: CPUControlFlags,
    pub size: u8,
    pub data: [u8; 124],
}

#[repr(C)]
pub struct ModInitial {
    pub freq_div: u32,
    pub data: [u8; MOD_HEADER_INITIAL_DATA_SIZE],
}

#[repr(C)]
pub struct ModSubsequent {
    pub data: [u8; MOD_HEADER_SUBSEQUENT_DATA_SIZE],
}

#[repr(C)]
pub struct SilencerHeader {
    pub cycle: u16,
    pub step: u16,
    _unused: [u8; 120],
}

impl GlobalHeader {
    pub fn new() -> Self {
        Self {
            msg_id: 0,
            fpga_flag: FPGAControlFlags::NONE,
            cpu_flag: CPUControlFlags::NONE,
            size: 0,
            data: [0x00; 124],
        }
    }

    pub fn mod_initial(&self) -> &ModInitial {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn mod_initial_mut(&mut self) -> &mut ModInitial {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn mod_subsequent(&self) -> &ModSubsequent {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn mod_subsequent_mut(&mut self) -> &mut ModSubsequent {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn silencer(&self) -> &SilencerHeader {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn silencer_mut(&mut self) -> &mut SilencerHeader {
        unsafe { std::mem::transmute(&mut self.data) }
    }
}

impl Default for GlobalHeader {
    fn default() -> Self {
        Self::new()
    }
}
