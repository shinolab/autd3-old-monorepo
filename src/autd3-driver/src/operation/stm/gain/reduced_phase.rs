/*
 * File: reduced_phase.rs
 * Project: gain
 * Created Date: 06/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{common::Drive, fpga::FPGADrive};

#[repr(C)]
pub struct PhaseFull<const N: usize> {
    phase_0: u8,
    phase_1: u8,
}

impl PhaseFull<0> {
    pub fn set(&mut self, d: &Drive) {
        self.phase_0 = FPGADrive::to_phase(d);
    }
}

impl PhaseFull<1> {
    pub fn set(&mut self, d: &Drive) {
        self.phase_1 = FPGADrive::to_phase(d);
    }
}

#[repr(C)]
pub struct PhaseHalf<const N: usize> {
    phase_01: u8,
    phase_23: u8,
}

impl PhaseHalf<0> {
    pub fn set(&mut self, d: &Drive) {
        let phase = FPGADrive::to_phase(d);
        self.phase_01 = (self.phase_01 & 0xF0) | ((phase >> 4) & 0x0F);
    }
}

impl PhaseHalf<1> {
    pub fn set(&mut self, d: &Drive) {
        let phase = FPGADrive::to_phase(d);
        self.phase_01 = (self.phase_01 & 0x0F) | (phase & 0xF0);
    }
}

impl PhaseHalf<2> {
    pub fn set(&mut self, d: &Drive) {
        let phase = FPGADrive::to_phase(d);
        self.phase_23 = (self.phase_23 & 0xF0) | ((phase >> 4) & 0x0F);
    }
}

impl PhaseHalf<3> {
    pub fn set(&mut self, d: &Drive) {
        let phase = FPGADrive::to_phase(d);
        self.phase_23 = (self.phase_23 & 0x0F) | (phase & 0xF0);
    }
}
