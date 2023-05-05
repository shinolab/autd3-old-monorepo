/*
 * File: body.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    float,
    fpga::{AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive},
    Drive, FOCUS_STM_FIXED_NUM_UNIT,
};

#[repr(C)]
pub struct STMFocus {
    pub(crate) buf: [u16; 4],
}

impl STMFocus {
    pub fn new(x: float, y: float, z: float, duty_shift: u8) -> Self {
        let x = (x / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let y = (y / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let z = (z / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let d0 = (x & 0xFFFF) as u16;
        let d1 =
            ((y << 2) & 0xFFFC) as u16 | ((x >> 30) & 0x0002) as u16 | ((x >> 16) & 0x0001) as u16;
        let d2 =
            ((z << 4) & 0xFFF0) as u16 | ((y >> 28) & 0x0008) as u16 | ((y >> 14) & 0x0007) as u16;
        let d3 = (((duty_shift as u16) << 6) & 0x3FC0)
            | ((z >> 26) & 0x0020) as u16
            | ((z >> 12) & 0x001F) as u16;
        Self {
            buf: [d0, d1, d2, d3],
        }
    }
}

pub const FOCUS_STM_BODY_INITIAL_SIZE: usize = 14;

#[repr(C)]
pub struct FocusSTMBodyInitial<T: ?Sized> {
    data: T,
}

impl FocusSTMBodyInitial<[u16]> {
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

pub const FOCUS_STM_BODY_SUBSEQUENT_SIZE: usize = 2;

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
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
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

impl GainSTMBodySubsequent<[u16]> {
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    pub fn legacy_drives_mut(&mut self) -> &mut [LegacyDrive] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn phases_mut(&mut self) -> &mut [AdvancedDrivePhase] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn duties_mut(&mut self) -> &mut [AdvancedDriveDuty] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn legacy_phase_full_mut<const N: usize>(&mut self) -> &mut [LegacyPhaseFull<N>] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

    pub fn legacy_phase_half_mut<const N: usize>(&mut self) -> &mut [LegacyPhaseHalf<N>] {
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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use rand::prelude::*;
    use std::mem::size_of;

    use super::*;
    use crate::PI;

    #[test]
    fn stm_focus() {
        assert_eq!(size_of::<STMFocus>(), 8);

        let max = ((1 << 17) - 1) as float * FOCUS_STM_FIXED_NUM_UNIT;
        let min = -(1 << 17) as float * FOCUS_STM_FIXED_NUM_UNIT;

        let mut rng = rand::thread_rng();

        let to = |v: u64| {
            let b = (v & 0x0003ffffu64) as u32;
            let b = if (v & 0x20000) == 0 {
                b
            } else {
                b | 0xfffc0000u32
            };
            unsafe { *(&b as *const _ as *const i32) as float * FOCUS_STM_FIXED_NUM_UNIT }
        };

        for _ in 0..10000 {
            let x = rng.gen_range(min..max);
            let y = rng.gen_range(min..max);
            let z = rng.gen_range(min..max);
            let shift: u8 = rng.gen_range(0..0xFF);

            let focus = STMFocus::new(x, y, z, shift);

            let mut v = 0u64;
            unsafe {
                std::ptr::copy_nonoverlapping(
                    &focus as *const _ as *const u64,
                    &mut v as *mut _,
                    1,
                );
            }

            let xx = to(v);
            assert_approx_eq!(xx, x, FOCUS_STM_FIXED_NUM_UNIT);

            let v = v >> 18;
            let yy = to(v);
            assert_approx_eq!(yy, y, FOCUS_STM_FIXED_NUM_UNIT);

            let v = v >> 18;
            let zz = to(v);
            assert_approx_eq!(zz, z, FOCUS_STM_FIXED_NUM_UNIT);

            let v = v >> 18;
            let s = (v & 0xFF) as u8;
            assert_eq!(s, shift);
        }
    }

    #[test]
    fn focus_stm_body_initial() {
        let point_size = 1000;
        let size = 7 * size_of::<u16>() + point_size * size_of::<STMFocus>();

        let mut d = vec![0x00u8; size];

        unsafe {
            let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
                as *mut FocusSTMBodyInitial<[u16]>);
            b.set_size(point_size as u16);
            b.set_freq_div(0x01234567);
            b.set_sound_speed(0x89ABCDEF);
            b.set_start_idx(0x7654);
            b.set_finish_idx(0x3210);

            let mut points = (0..point_size)
                .map(|_| STMFocus::new(0., 0., 0., 0))
                .collect::<Vec<_>>();
            let buf = (0..point_size * size_of::<STMFocus>())
                .map(|i| i as u8)
                .collect::<Vec<_>>();
            std::ptr::copy_nonoverlapping(buf.as_ptr(), points.as_mut_ptr() as *mut _, buf.len());
            (*b).set_points(&points);
        }

        assert_eq!(d[0], (point_size & 0xFF) as _);
        assert_eq!(d[1], (point_size >> 8) as _);
        assert_eq!(d[2], 0x67);
        assert_eq!(d[3], 0x45);
        assert_eq!(d[4], 0x23);
        assert_eq!(d[5], 0x01);
        assert_eq!(d[6], 0xEF);
        assert_eq!(d[7], 0xCD);
        assert_eq!(d[8], 0xAB);
        assert_eq!(d[9], 0x89);
        assert_eq!(d[10], 0x54);
        assert_eq!(d[11], 0x76);
        assert_eq!(d[12], 0x10);
        assert_eq!(d[13], 0x32);
        for i in 0..point_size * size_of::<STMFocus>() {
            assert_eq!(d[14 + i], i as u8);
        }
    }

    #[test]
    fn focus_stm_body_subsequent() {
        let point_size = 1000;
        let size = 1 * size_of::<u16>() + point_size * size_of::<STMFocus>();

        let mut d = vec![0x00u8; size];

        unsafe {
            let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
                as *mut FocusSTMBodySubsequent<[u16]>);
            b.set_size(point_size as u16);

            let mut points = (0..point_size)
                .map(|_| STMFocus::new(0., 0., 0., 0))
                .collect::<Vec<_>>();
            let buf = (0..point_size * size_of::<STMFocus>())
                .map(|i| i as u8)
                .collect::<Vec<_>>();
            std::ptr::copy_nonoverlapping(buf.as_ptr(), points.as_mut_ptr() as *mut _, buf.len());
            (*b).set_points(&points);
        }

        assert_eq!(d[0], (point_size & 0xFF) as _);
        assert_eq!(d[1], (point_size >> 8) as _);
        for i in 0..point_size * size_of::<STMFocus>() {
            assert_eq!(d[2 + i], i as u8);
        }
    }

    #[test]
    fn legacy_phase_full() {
        assert_eq!(size_of::<LegacyPhaseFull<0>>(), 2);
        assert_eq!(size_of::<LegacyPhaseFull<1>>(), 2);

        let mut p = vec![0x00u8; 2];
        let mut s = Drive { amp: 0., phase: 0. };

        s.phase = PI;
        unsafe {
            (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseFull<0>))
                .set(&s);
        }
        let expect_phase_0 = LegacyDrive::to_phase(&s);
        assert_eq!(p[0], expect_phase_0);
        assert_eq!(p[1], 0);

        s.phase = 1.5 * PI;
        unsafe {
            (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseFull<1>))
                .set(&s);
        }
        let expect_phase_1 = LegacyDrive::to_phase(&s);
        assert_eq!(p[0], expect_phase_0);
        assert_eq!(p[1], expect_phase_1);

        s.phase = 0.;
        unsafe {
            (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseFull<0>))
                .set(&s);
        }
        assert_eq!(p[0], 0);
        assert_eq!(p[1], expect_phase_1);
    }

    #[test]
    fn legacy_phase_half() {
        assert_eq!(size_of::<LegacyPhaseHalf<0>>(), 2);
        assert_eq!(size_of::<LegacyPhaseHalf<1>>(), 2);
        assert_eq!(size_of::<LegacyPhaseHalf<2>>(), 2);
        assert_eq!(size_of::<LegacyPhaseHalf<3>>(), 2);

        let mut p = vec![0x00u8; 2];
        let mut s = Drive { amp: 0., phase: 0. };

        s.phase = PI;
        unsafe {
            (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseHalf<0>))
                .set(&s);
        }
        let expect_phase_0 = LegacyDrive::to_phase(&s) >> 4;
        assert_eq!(p[0] & 0x0F, expect_phase_0);
        assert_eq!(p[0] & 0xF0, 0);
        assert_eq!(p[1] & 0x0F, 0);
        assert_eq!(p[1] & 0xF0, 0);

        s.phase = 1.5 * PI;
        unsafe {
            (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseHalf<1>))
                .set(&s);
        }
        let expect_phase_1 = LegacyDrive::to_phase(&s) >> 4;
        assert_eq!(p[0] & 0x0F, expect_phase_0);
        assert_eq!(p[0] & 0xF0, expect_phase_1 << 4);
        assert_eq!(p[1] & 0x0F, 0);
        assert_eq!(p[1] & 0xF0, 0);

        s.phase = 0.8 * PI;
        unsafe {
            (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseHalf<2>))
                .set(&s);
        }
        let expect_phase_2 = LegacyDrive::to_phase(&s) >> 4;
        assert_eq!(p[0] & 0x0F, expect_phase_0);
        assert_eq!(p[0] & 0xF0, expect_phase_1 << 4);
        assert_eq!(p[1] & 0x0F, expect_phase_2);
        assert_eq!(p[1] & 0xF0, 0);

        s.phase = 1.2 * PI;
        unsafe {
            (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseHalf<3>))
                .set(&s);
        }
        let expect_phase_3 = LegacyDrive::to_phase(&s) >> 4;
        assert_eq!(p[0] & 0x0F, expect_phase_0);
        assert_eq!(p[0] & 0xF0, expect_phase_1 << 4);
        assert_eq!(p[1] & 0x0F, expect_phase_2);
        assert_eq!(p[1] & 0xF0, expect_phase_3 << 4);
    }

    #[test]
    fn gain_stm_body_initial() {
        let size = 6 * size_of::<u16>();

        let mut d = vec![0x00u8; size];

        unsafe {
            let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
                as *mut GainSTMBodyInitial<[u16]>);
            b.set_freq_div(0x01234567);
            b.set_mode(Mode::PhaseDutyFull);
            b.set_cycle(0x89AB);
            b.set_start_idx(0x7654);
            b.set_finish_idx(0x3210);
        }

        assert_eq!(d[0], 0x67);
        assert_eq!(d[1], 0x45);
        assert_eq!(d[2], 0x23);
        assert_eq!(d[3], 0x01);
        assert_eq!(d[4], Mode::PhaseDutyFull as _);
        assert_eq!(d[5], 0x00);
        assert_eq!(d[6], 0xAB);
        assert_eq!(d[7], 0x89);
        assert_eq!(d[8], 0x54);
        assert_eq!(d[9], 0x76);
        assert_eq!(d[10], 0x10);
        assert_eq!(d[11], 0x32);

        unsafe {
            let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
                as *mut GainSTMBodyInitial<[u16]>);
            b.set_mode(Mode::PhaseFull);
        }
        assert_eq!(d[4], Mode::PhaseFull as _);

        unsafe {
            let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
                as *mut GainSTMBodyInitial<[u16]>);
            b.set_mode(Mode::PhaseHalf);
        }
        assert_eq!(d[4], Mode::PhaseHalf as _);
    }
}
