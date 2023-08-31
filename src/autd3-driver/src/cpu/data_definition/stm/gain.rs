/*
 * File: gain.rs
 * Project: stm
 * Created Date: 30/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{cpu::data_definition::TypeTag, Drive, LegacyDrive};

use super::STMControlFlags;

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    PhaseDutyFull = 0x0001,
    PhaseFull = 0x0002,
    PhaseHalf = 0x0004,
}

#[repr(C)]
pub struct LegacyPhaseFull<const N: usize> {
    phase_0: u8,
    phase_1: u8,
}

impl LegacyPhaseFull<0> {
    pub fn set(&mut self, d: &Drive) {
        self.phase_0 = LegacyDrive::to_phase(d);
    }
}

impl LegacyPhaseFull<1> {
    pub fn set(&mut self, d: &Drive) {
        self.phase_1 = LegacyDrive::to_phase(d);
    }
}

#[repr(C)]
pub struct LegacyPhaseHalf<const N: usize> {
    phase_01: u8,
    phase_23: u8,
}

impl LegacyPhaseHalf<0> {
    pub fn set(&mut self, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        self.phase_01 = (self.phase_01 & 0xF0) | ((phase >> 4) & 0x0F);
    }
}

impl LegacyPhaseHalf<1> {
    pub fn set(&mut self, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        self.phase_01 = (self.phase_01 & 0x0F) | (phase & 0xF0);
    }
}

impl LegacyPhaseHalf<2> {
    pub fn set(&mut self, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        self.phase_23 = (self.phase_23 & 0xF0) | ((phase >> 4) & 0x0F);
    }
}

impl LegacyPhaseHalf<3> {
    pub fn set(&mut self, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        self.phase_23 = (self.phase_23 & 0x0F) | (phase & 0xF0);
    }
}

pub struct GainSTMInitial {}

impl GainSTMInitial {
    fn write<T>(
        tx: &mut [u16],
        freq_div: u32,
        mode: Mode,
        cycle: usize,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
        drives: &[T],
    ) {
        let mut f = STMControlFlags::NONE;
        f.set(STMControlFlags::USE_START_IDX, start_idx.is_some());
        f.set(STMControlFlags::USE_FINISH_IDX, finish_idx.is_some());
        tx[0] = (f.bits() as u16) << 8 | TypeTag::GainSTMInitial as u16;

        tx[1] = (freq_div & 0x0000FFFF) as _;
        tx[2] = ((freq_div >> 16) & 0x0000FFFF) as _;

        tx[3] = mode as u16;

        tx[4] = cycle as u16;

        tx[5] = start_idx.unwrap_or(0);
        tx[6] = finish_idx.unwrap_or(0);

        unsafe {
            std::ptr::copy_nonoverlapping(
                drives.as_ptr() as *const u16,
                tx[7..].as_mut_ptr() as *mut u16,
                drives.len(),
            )
        }
    }
}

pub struct GainSTMSubsequent {}

impl GainSTMSubsequent {
    fn write<T>(
        tx: &mut [u16],
        freq_div: u32,
        mode: Mode,
        cycle: usize,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
        drives: &[T],
    ) {
        tx[0] = TypeTag::GainSTMInitial as u16;
        unsafe {
            std::ptr::copy_nonoverlapping(
                drives.as_ptr() as *const u16,
                tx[1..].as_mut_ptr() as *mut u16,
                drives.len(),
            )
        }
    }
}
