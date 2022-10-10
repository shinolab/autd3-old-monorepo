/*
 * File: fpga_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

pub const FPGA_CLK_FREQ: usize = 163840000;

pub const MAX_CYCLE: u16 = 8191;

pub const MOD_SAMPLING_FREQ_DIV_MIN: u32 = 1160;
pub const MOD_BUF_SIZE_MAX: usize = 65536;

pub const POINT_STM_FIXED_NUM_UNIT: f64 = 0.025; //mm

pub const STM_SAMPLING_FREQ_DIV_MIN: u32 = 1612;
pub const POINT_STM_BUF_SIZE_MAX: usize = 65536;
pub const GAIN_STM_NORMAL_BUF_SIZE_MAX: usize = 1024;
pub const GAIN_STM_LEGACY_BUF_SIZE_MAX: usize = 2048;

pub const SILENCER_CYCLE_MIN: u16 = 1044;

bitflags::bitflags! {
    pub struct FPGAControlFlags : u8 {
        const NONE            = 0;
        const LEGACY_MODE     = 1 << 0;
        const FORCE_FAN       = 1 << 4;
        const STM_MODE        = 1 << 5;
        const STM_GAIN_MODE   = 1 << 6;
        const READS_FPGA_INFO = 1 << 7;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Drive {
    pub phase: f64,
    pub amp: f64,
    pub cycle: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct LegacyDrive {
    pub phase: u8,
    pub duty: u8,
}

impl LegacyDrive {
    pub fn to_phase(d: &Drive) -> u8 {
        (((d.phase * 256.0).round() as i32) & 0xFF) as u8
    }

    pub fn set(&mut self, d: &Drive) {
        self.duty = (510.0 * d.amp.asin() / PI).round() as u8;
        self.phase = Self::to_phase(d);
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Phase {
    pub phase: u16,
}

impl Phase {
    pub fn set(&mut self, d: &Drive) {
        self.phase = ((d.phase * d.cycle as f64).round() as i32).rem_euclid(d.cycle as i32) as _;
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Duty {
    pub duty: u16,
}

impl Duty {
    pub fn set(&mut self, d: &Drive) {
        self.duty = (d.cycle as f64 * d.amp.asin() / PI).round() as _;
    }
}

#[derive(Default)]
#[repr(C)]
pub struct FPGAInfo {
    info: u8,
}

impl FPGAInfo {
    pub fn is_thermal_assert(&self) -> bool {
        (self.info & 0x01) != 0
    }
}
