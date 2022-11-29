/*
 * File: header.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    cpu::{CPUControlFlags, MOD_HEADER_INITIAL_DATA_SIZE, MOD_HEADER_SUBSEQUENT_DATA_SIZE},
    fpga::FPGAControlFlags,
};

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
pub struct MOD_HEAD {
    pub freq_div: u32,
    pub data: [u8; MOD_HEADER_INITIAL_DATA_SIZE],
}

#[repr(C)]
pub struct MOD_BODY {
    pub data: [u8; MOD_HEADER_SUBSEQUENT_DATA_SIZE],
}

#[repr(C)]
pub struct SILENCER_HEADER {
    pub cycle: u16,
    pub step: u16,
    _data: [u8; 120],
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

    pub fn mod_initial(&self) -> &MOD_HEAD {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn mod_initial_mut(&mut self) -> &mut MOD_HEAD {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn mod_subsequent(&self) -> &MOD_BODY {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn mod_subsequent_mut(&mut self) -> &mut MOD_BODY {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn silencer(&self) -> &SILENCER_HEADER {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn silencer_mut(&mut self) -> &mut SILENCER_HEADER {
        unsafe { std::mem::transmute(&mut self.data) }
    }
}

impl Default for GlobalHeader {
    fn default() -> Self {
        Self::new()
    }
}
