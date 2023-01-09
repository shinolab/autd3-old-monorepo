/*
 * File: fpga_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

pub const FPGA_CLK_FREQ: usize = 163840000;

pub const FOCUS_STM_FIXED_NUM_UNIT: f64 = 0.025; //mm

#[derive(Clone, Copy, Debug)]
pub struct Drive {
    pub phase: f64,
    pub amp: f64,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct LegacyDrive {
    pub phase: u8,
    pub duty: u8,
}

impl LegacyDrive {
    pub fn to_phase(d: &Drive) -> u8 {
        (((d.phase / (2.0 * PI) * 256.0).round() as i32) & 0xFF) as _
    }

    pub fn to_duty(d: &Drive) -> u8 {
        (510.0 * d.amp.clamp(0., 1.).asin() / PI).round() as _
    }

    pub fn set(&mut self, d: &Drive) {
        self.duty = Self::to_duty(d);
        self.phase = Self::to_phase(d);
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Phase {
    pub phase: u16,
}

impl Phase {
    pub fn to_phase(d: &Drive, cycle: u16) -> u16 {
        ((d.phase / (2.0 * PI) * cycle as f64).round() as i32).rem_euclid(cycle as i32) as _
    }

    pub fn set(&mut self, d: &Drive, cycle: u16) {
        self.phase = Self::to_phase(d, cycle);
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Duty {
    pub duty: u16,
}

impl Duty {
    pub fn to_duty(d: &Drive, cycle: u16) -> u16 {
        (cycle as f64 * d.amp.clamp(0., 1.).asin() / PI).round() as _
    }

    pub fn set(&mut self, d: &Drive, cycle: u16) {
        self.duty = Self::to_duty(d, cycle);
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
