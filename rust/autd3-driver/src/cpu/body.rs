/*
 * File: body.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    fpga::{Duty, LegacyDrive, Phase},
    hardware::NUM_TRANS_IN_UNIT,
    Drive, Mode, POINT_STM_FIXED_NUM_UNIT,
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Body {
    pub data: [u16; NUM_TRANS_IN_UNIT],
}

impl Body {
    pub fn new() -> Self {
        Self {
            data: [0x0000; NUM_TRANS_IN_UNIT],
        }
    }

    pub fn legacy_drives_mut(&mut self) -> &mut [LegacyDrive; NUM_TRANS_IN_UNIT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn duties_mut(&mut self) -> &mut [Duty; NUM_TRANS_IN_UNIT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn phases_mut(&mut self) -> &mut [Phase; NUM_TRANS_IN_UNIT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn point_stm_head(&self) -> &PointSTMBodyHead {
        unsafe { std::mem::transmute(self) }
    }

    pub fn point_stm_head_mut(&mut self) -> &mut PointSTMBodyHead {
        unsafe { std::mem::transmute(self) }
    }

    pub fn point_stm_body(&self) -> &PointSTMBodyBody {
        unsafe { std::mem::transmute(self) }
    }

    pub fn point_stm_body_mut(&mut self) -> &mut PointSTMBodyBody {
        unsafe { std::mem::transmute(self) }
    }

    pub fn gain_stm_head(&self) -> &GainSTMBodyHead {
        unsafe { std::mem::transmute(self) }
    }

    pub fn gain_stm_head_mut(&mut self) -> &mut GainSTMBodyHead {
        unsafe { std::mem::transmute(self) }
    }

    pub fn gain_stm_body(&self) -> &GainSTMBodyBody {
        unsafe { std::mem::transmute(self) }
    }

    pub fn gain_stm_body_mut(&mut self) -> &mut GainSTMBodyBody {
        unsafe { std::mem::transmute(self) }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
pub struct SeqFocus {
    pub(crate) buf: [u16; 4],
}

impl SeqFocus {
    pub fn new(x: f64, y: f64, z: f64, duty_shift: u8) -> Self {
        let x = (x / POINT_STM_FIXED_NUM_UNIT).round() as i32;
        let y = (y / POINT_STM_FIXED_NUM_UNIT).round() as i32;
        let z = (z / POINT_STM_FIXED_NUM_UNIT).round() as i32;
        let d0 = (x & 0xFFFF) as u16;
        let d1 =
            ((y << 2) & 0xFFFC) as u16 | ((x >> 30) & 0x0002) as u16 | ((x >> 16) & 0x0001) as u16;
        let d2 =
            ((z << 4) & 0xFFF0) as u16 | ((y >> 28) & 0x0008) as u16 | ((y >> 14) & 0x0007) as u16;
        let d3 = (((duty_shift as u16) << 6) & 0x3FC0) as u16
            | ((z >> 26) & 0x0020) as u16
            | ((z >> 12) & 0x001F) as u16;
        SeqFocus {
            buf: [d0, d1, d2, d3],
        }
    }
}

#[repr(C)]
pub struct PointSTMBodyHead {
    data: [u16; NUM_TRANS_IN_UNIT],
}

impl PointSTMBodyHead {
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

    pub fn set_points(&mut self, points: &[SeqFocus]) {
        self.data[5..]
            .chunks_mut(4)
            .zip(points.iter())
            .for_each(|(d, s)| d.copy_from_slice(&s.buf));
    }
}

#[repr(C)]
pub struct PointSTMBodyBody {
    data: [u16; NUM_TRANS_IN_UNIT],
}

impl PointSTMBodyBody {
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    pub fn set_size(&mut self, size: u16) {
        self.data[0] = size;
    }

    pub fn set_points(&mut self, points: &[SeqFocus]) {
        self.data[1..]
            .chunks_mut(4)
            .zip(points.iter())
            .for_each(|(d, s)| d.copy_from_slice(&s.buf));
    }
}

#[repr(C)]
pub struct GainSTMBodyHead {
    data: [u16; NUM_TRANS_IN_UNIT],
}

impl GainSTMBodyHead {
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
}

#[repr(C)]
pub struct GainSTMBodyBody {
    data: [u16; NUM_TRANS_IN_UNIT],
}

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

impl GainSTMBodyBody {
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    pub fn legacy_drives_mut(&mut self) -> &mut [LegacyDrive; NUM_TRANS_IN_UNIT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn phases_mut(&mut self) -> &mut [Phase; NUM_TRANS_IN_UNIT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn duties_mut(&mut self) -> &mut [Duty; NUM_TRANS_IN_UNIT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn legacy_phase_full_mut(&mut self) -> &mut [LegacyPhaseFull; NUM_TRANS_IN_UNIT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn legacy_phase_half_mut(&mut self) -> &mut [LegacyPhaseHalf; NUM_TRANS_IN_UNIT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }
}
