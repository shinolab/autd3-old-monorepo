/*
 * File: body.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    fpga::{Duty, LegacyDrive, Phase},
    Drive, FOCUS_STM_FIXED_NUM_UNIT,
};

#[repr(C)]
pub struct STMFocus {
    pub(crate) buf: [u16; 4],
}

impl STMFocus {
    pub fn new(x: f64, y: f64, z: f64, duty_shift: u8) -> Self {
        let x = (x / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let y = (y / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let z = (z / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let d0 = (x & 0xFFFF) as u16;
        let d1 =
            ((y << 2) & 0xFFFC) as u16 | ((x >> 30) & 0x0002) as u16 | ((x >> 16) & 0x0001) as u16;
        let d2 =
            ((z << 4) & 0xFFF0) as u16 | ((y >> 28) & 0x0008) as u16 | ((y >> 14) & 0x0007) as u16;
        let d3 = (((duty_shift as u16) << 6) & 0x3FC0) as u16
            | ((z >> 26) & 0x0020) as u16
            | ((z >> 12) & 0x001F) as u16;
        Self {
            buf: [d0, d1, d2, d3],
        }
    }
}

#[repr(C)]
pub struct FocusSTMBodyInitial<T: ?Sized> {
    data: T,
}

impl FocusSTMBodyInitial<[u16]> {
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    pub fn set_size(&mut self, size: u16) {
        self.data[0] = size;
    }

    pub fn set_freq_div(&mut self, freq_div: u32) {
        self.data[1] = (freq_div & 0x0000FFFF) as _;
        self.data[2] = ((freq_div >> 16) & 0x0000FFFF) as _;
    }

    pub fn set_sound_speed(&mut self, sound_speed: u32) {
        self.data[3] = (sound_speed & 0x0000FFFF) as _;
        self.data[4] = ((sound_speed >> 16) & 0x0000FFFF) as _;
    }

    pub fn set_start_idx(&mut self, idx: u16) {
        self.data[5] = idx;
    }

    pub fn set_finish_idx(&mut self, idx: u16) {
        self.data[6] = idx;
    }

    pub fn set_points(&mut self, points: &[STMFocus]) {
        self.data[7..]
            .chunks_mut(std::mem::size_of::<STMFocus>() / std::mem::size_of::<u16>())
            .zip(points.iter())
            .for_each(|(d, s)| d.copy_from_slice(&s.buf));
    }
}

#[repr(C)]
pub struct FocusSTMBodySubsequent<T: ?Sized> {
    data: T,
}

impl FocusSTMBodySubsequent<[u16]> {
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    pub fn set_size(&mut self, size: u16) {
        self.data[0] = size;
    }

    pub fn set_points(&mut self, points: &[STMFocus]) {
        self.data[1..]
            .chunks_mut(std::mem::size_of::<STMFocus>() / std::mem::size_of::<u16>())
            .zip(points.iter())
            .for_each(|(d, s)| d.copy_from_slice(&s.buf));
    }
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    PhaseDutyFull = 0x0001,
    PhaseFull = 0x0002,
    PhaseHalf = 0x0004,
}

#[repr(C)]
pub struct GainSTMBodyInitial<T: ?Sized> {
    data: T,
}

impl GainSTMBodyInitial<[u16]> {
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    pub fn set_freq_div(&mut self, freq_div: u32) {
        self.data[0] = (freq_div & 0x0000FFFF) as _;
        self.data[1] = ((freq_div >> 16) & 0x0000FFFF) as _;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.data[2] = mode as u16;
    }

    pub fn set_cycle(&mut self, cycle: usize) {
        self.data[3] = cycle as u16;
    }

    pub fn set_start_idx(&mut self, idx: u16) {
        self.data[4] = idx;
    }

    pub fn set_finish_idx(&mut self, idx: u16) {
        self.data[5] = idx;
    }
}

#[repr(C)]
pub struct GainSTMBodySubsequent<T: ?Sized> {
    data: T,
}

#[repr(C)]
pub struct LegacyPhaseFull {
    phase_0: u8,
    phase_1: u8,
}

impl LegacyPhaseFull {
    pub fn set(&mut self, idx: usize, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        match idx {
            0 => self.phase_0 = phase,
            1 => self.phase_1 = phase,
            _ => unreachable!(),
        }
    }
}

#[repr(C)]
pub struct LegacyPhaseHalf {
    phase_01: u8,
    phase_23: u8,
}

impl LegacyPhaseHalf {
    pub fn set(&mut self, idx: usize, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        match idx {
            0 => self.phase_01 = (self.phase_01 & 0xF0) | ((phase >> 4) & 0x0F),
            1 => self.phase_01 = (self.phase_01 & 0x0F) | (phase & 0xF0),
            2 => self.phase_23 = (self.phase_23 & 0xF0) | ((phase >> 4) & 0x0F),
            3 => self.phase_23 = (self.phase_23 & 0x0F) | (phase & 0xF0),
            _ => unreachable!(),
        }
    }
}

impl GainSTMBodySubsequent<[u16]> {
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    pub fn legacy_drives_mut(&mut self) -> &mut [LegacyDrive] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn phases_mut(&mut self) -> &mut [Phase] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn duties_mut(&mut self) -> &mut [Duty] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn legacy_phase_full_mut(&mut self) -> &mut [LegacyPhaseFull] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn legacy_phase_half_mut(&mut self) -> &mut [LegacyPhaseHalf] {
        unsafe { std::mem::transmute(&mut self.data) }
    }
}

#[repr(C)]
pub struct Body<T: ?Sized> {
    data: T,
}

impl Body<[u16]> {
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    pub fn focus_stm_initial(&self) -> &FocusSTMBodyInitial<[u16]> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn focus_stm_initial_mut(&mut self) -> &mut FocusSTMBodyInitial<[u16]> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn focus_stm_subsequent(&self) -> &FocusSTMBodySubsequent<[u16]> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn focus_stm_subsequent_mut(&mut self) -> &mut FocusSTMBodySubsequent<[u16]> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn gain_stm_initial(&self) -> &GainSTMBodyInitial<[u16]> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn gain_stm_initial_mut(&mut self) -> &mut GainSTMBodyInitial<[u16]> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn gain_stm_subsequent(&self) -> &GainSTMBodySubsequent<[u16]> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn gain_stm_subsequent_mut(&mut self) -> &mut GainSTMBodySubsequent<[u16]> {
        unsafe { std::mem::transmute(self) }
    }
}
