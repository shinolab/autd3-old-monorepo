/*
 * File: focus.rs
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

use crate::{cpu::data_definition::TypeTag, float, STMFocus, METER};

use super::STMControlFlags;

pub struct FocusSTMInitial {}

impl FocusSTMInitial {
    pub fn write(
        tx: &mut [u16],
        freq_div: u32,
        sound_speed: float,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
        points: &[STMFocus],
    ) {
        let mut f = STMControlFlags::NONE;
        f.set(STMControlFlags::USE_START_IDX, start_idx.is_some());
        f.set(STMControlFlags::USE_FINISH_IDX, finish_idx.is_some());
        tx[0] = (f.bits() as u16) << 8 | TypeTag::FocusSTMInitial as u16;

        let size = points.len();
        tx[1] = size as u16;

        tx[2] = (freq_div & 0x0000FFFF) as _;
        tx[3] = ((freq_div >> 16) & 0x0000FFFF) as _;

        let sound_speed = (sound_speed / METER * 1024.0).round() as u32;
        tx[4] = (sound_speed & 0x0000FFFF) as _;
        tx[5] = ((sound_speed >> 16) & 0x0000FFFF) as _;

        tx[6] = start_idx.unwrap_or(0);
        tx[7] = finish_idx.unwrap_or(0);

        unsafe {
            std::ptr::copy_nonoverlapping(
                points.as_ptr(),
                (&mut tx[8..]).as_mut_ptr() as *mut STMFocus,
                points.len(),
            )
        }
    }
}

pub struct FocusSTMSubsequent {}

impl FocusSTMSubsequent {
    pub fn write(tx: &mut [u16], points: &[STMFocus]) {
        tx[0] = TypeTag::FocusSTMSubsequent as u16;

        let size = points.len();
        tx[1] = size as u16;

        unsafe {
            std::ptr::copy_nonoverlapping(
                points.as_ptr(),
                (&mut tx[2..]).as_mut_ptr() as *mut STMFocus,
                points.len(),
            )
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use assert_approx_eq::assert_approx_eq;
//     use rand::prelude::*;
//     use std::mem::size_of;

//     use super::*;
//     use crate::PI;

//     #[test]
//     fn stm_focus() {
//         assert_eq!(size_of::<STMFocus>(), 8);

//         let max = ((1 << 17) - 1) as float * FOCUS_STM_FIXED_NUM_UNIT;
//         let min = -(1 << 17) as float * FOCUS_STM_FIXED_NUM_UNIT;

//         let mut rng = rand::thread_rng();

//         let to = |v: u64| {
//             let b = (v & 0x0003ffffu64) as u32;
//             let b = if (v & 0x20000) == 0 {
//                 b
//             } else {
//                 b | 0xfffc0000u32
//             };
//             unsafe { *(&b as *const _ as *const i32) as float * FOCUS_STM_FIXED_NUM_UNIT }
//         };

//         (0..10000).for_each(|_| {
//             let x = rng.gen_range(min..max);
//             let y = rng.gen_range(min..max);
//             let z = rng.gen_range(min..max);
//             let shift: u8 = rng.gen_range(0..0xFF);

//             let focus = STMFocus::new(x, y, z, shift);

//             let mut v = 0u64;
//             unsafe {
//                 std::ptr::copy_nonoverlapping(
//                     &focus as *const _ as *const u64,
//                     &mut v as *mut _,
//                     1,
//                 );
//             }

//             let xx = to(v);
//             assert_approx_eq!(xx, x, FOCUS_STM_FIXED_NUM_UNIT);

//             let v = v >> 18;
//             let yy = to(v);
//             assert_approx_eq!(yy, y, FOCUS_STM_FIXED_NUM_UNIT);

//             let v = v >> 18;
//             let zz = to(v);
//             assert_approx_eq!(zz, z, FOCUS_STM_FIXED_NUM_UNIT);

//             let v = v >> 18;
//             let s = (v & 0xFF) as u8;
//             assert_eq!(s, shift);
//         });
//     }

//     #[test]
//     fn focus_stm_body_initial() {
//         let point_size = 1000;
//         let size = 7 * size_of::<u16>() + point_size * size_of::<STMFocus>();

//         let mut d = vec![0x00u8; size];

//         unsafe {
//             let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
//                 as *mut FocusSTMBodyInitial<[u16]>);
//             b.set_size(point_size as u16);
//             b.set_freq_div(0x01234567);
//             b.set_sound_speed(0x89ABCDEF);
//             b.set_start_idx(0x7654);
//             b.set_finish_idx(0x3210);

//             let mut points = (0..point_size)
//                 .map(|_| STMFocus::new(0., 0., 0., 0))
//                 .collect::<Vec<_>>();
//             let buf = (0..point_size * size_of::<STMFocus>())
//                 .map(|i| i as u8)
//                 .collect::<Vec<_>>();
//             std::ptr::copy_nonoverlapping(buf.as_ptr(), points.as_mut_ptr() as *mut _, buf.len());
//             (*b).set_points(&points);
//         }

//         assert_eq!(d[0], (point_size & 0xFF) as _);
//         assert_eq!(d[1], (point_size >> 8) as _);
//         assert_eq!(d[2], 0x67);
//         assert_eq!(d[3], 0x45);
//         assert_eq!(d[4], 0x23);
//         assert_eq!(d[5], 0x01);
//         assert_eq!(d[6], 0xEF);
//         assert_eq!(d[7], 0xCD);
//         assert_eq!(d[8], 0xAB);
//         assert_eq!(d[9], 0x89);
//         assert_eq!(d[10], 0x54);
//         assert_eq!(d[11], 0x76);
//         assert_eq!(d[12], 0x10);
//         assert_eq!(d[13], 0x32);
//         (0..point_size * size_of::<STMFocus>()).for_each(|i| {
//             assert_eq!(d[14 + i], i as u8);
//         });
//     }

//     #[test]
//     fn focus_stm_body_subsequent() {
//         let point_size = 1000;
//         let size = size_of::<u16>() + point_size * size_of::<STMFocus>();

//         let mut d = vec![0x00u8; size];

//         unsafe {
//             let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
//                 as *mut FocusSTMBodySubsequent<[u16]>);
//             b.set_size(point_size as u16);

//             let mut points = (0..point_size)
//                 .map(|_| STMFocus::new(0., 0., 0., 0))
//                 .collect::<Vec<_>>();
//             let buf = (0..point_size * size_of::<STMFocus>())
//                 .map(|i| i as u8)
//                 .collect::<Vec<_>>();
//             std::ptr::copy_nonoverlapping(buf.as_ptr(), points.as_mut_ptr() as *mut _, buf.len());
//             (*b).set_points(&points);
//         }

//         assert_eq!(d[0], (point_size & 0xFF) as _);
//         assert_eq!(d[1], (point_size >> 8) as _);
//         (0..point_size * size_of::<STMFocus>()).for_each(|i| {
//             assert_eq!(d[2 + i], i as u8);
//         });
//     }

//     #[test]
//     fn legacy_phase_full() {
//         assert_eq!(size_of::<LegacyPhaseFull<0>>(), 2);
//         assert_eq!(size_of::<LegacyPhaseFull<1>>(), 2);

//         let mut p = vec![0x00u8; 2];
//         let mut s = Drive { amp: 0., phase: 0. };

//         s.phase = PI;
//         unsafe {
//             (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseFull<0>))
//                 .set(&s);
//         }
//         let expect_phase_0 = LegacyDrive::to_phase(&s);
//         assert_eq!(p[0], expect_phase_0);
//         assert_eq!(p[1], 0);

//         s.phase = 1.5 * PI;
//         unsafe {
//             (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseFull<1>))
//                 .set(&s);
//         }
//         let expect_phase_1 = LegacyDrive::to_phase(&s);
//         assert_eq!(p[0], expect_phase_0);
//         assert_eq!(p[1], expect_phase_1);

//         s.phase = 0.;
//         unsafe {
//             (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseFull<0>))
//                 .set(&s);
//         }
//         assert_eq!(p[0], 0);
//         assert_eq!(p[1], expect_phase_1);
//     }

//     #[test]
//     fn legacy_phase_half() {
//         assert_eq!(size_of::<LegacyPhaseHalf<0>>(), 2);
//         assert_eq!(size_of::<LegacyPhaseHalf<1>>(), 2);
//         assert_eq!(size_of::<LegacyPhaseHalf<2>>(), 2);
//         assert_eq!(size_of::<LegacyPhaseHalf<3>>(), 2);

//         let mut p = vec![0x00u8; 2];
//         let mut s = Drive { amp: 0., phase: 0. };

//         s.phase = PI;
//         unsafe {
//             (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseHalf<0>))
//                 .set(&s);
//         }
//         let expect_phase_0 = LegacyDrive::to_phase(&s) >> 4;
//         assert_eq!(p[0] & 0x0F, expect_phase_0);
//         assert_eq!(p[0] & 0xF0, 0);
//         assert_eq!(p[1] & 0x0F, 0);
//         assert_eq!(p[1] & 0xF0, 0);

//         s.phase = 1.5 * PI;
//         unsafe {
//             (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseHalf<1>))
//                 .set(&s);
//         }
//         let expect_phase_1 = LegacyDrive::to_phase(&s) >> 4;
//         assert_eq!(p[0] & 0x0F, expect_phase_0);
//         assert_eq!(p[0] & 0xF0, expect_phase_1 << 4);
//         assert_eq!(p[1] & 0x0F, 0);
//         assert_eq!(p[1] & 0xF0, 0);

//         s.phase = 0.8 * PI;
//         unsafe {
//             (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseHalf<2>))
//                 .set(&s);
//         }
//         let expect_phase_2 = LegacyDrive::to_phase(&s) >> 4;
//         assert_eq!(p[0] & 0x0F, expect_phase_0);
//         assert_eq!(p[0] & 0xF0, expect_phase_1 << 4);
//         assert_eq!(p[1] & 0x0F, expect_phase_2);
//         assert_eq!(p[1] & 0xF0, 0);

//         s.phase = 1.2 * PI;
//         unsafe {
//             (*(std::ptr::slice_from_raw_parts_mut(p.as_mut_ptr(), 2) as *mut LegacyPhaseHalf<3>))
//                 .set(&s);
//         }
//         let expect_phase_3 = LegacyDrive::to_phase(&s) >> 4;
//         assert_eq!(p[0] & 0x0F, expect_phase_0);
//         assert_eq!(p[0] & 0xF0, expect_phase_1 << 4);
//         assert_eq!(p[1] & 0x0F, expect_phase_2);
//         assert_eq!(p[1] & 0xF0, expect_phase_3 << 4);
//     }

//     #[test]
//     fn gain_stm_body_initial() {
//         let size = 6 * size_of::<u16>();

//         let mut d = vec![0x00u8; size];

//         unsafe {
//             let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
//                 as *mut GainSTMBodyInitial<[u16]>);
//             b.set_freq_div(0x01234567);
//             b.set_mode(Mode::PhaseDutyFull);
//             b.set_cycle(0x89AB);
//             b.set_start_idx(0x7654);
//             b.set_finish_idx(0x3210);
//         }

//         assert_eq!(d[0], 0x67);
//         assert_eq!(d[1], 0x45);
//         assert_eq!(d[2], 0x23);
//         assert_eq!(d[3], 0x01);
//         assert_eq!(d[4], Mode::PhaseDutyFull as _);
//         assert_eq!(d[5], 0x00);
//         assert_eq!(d[6], 0xAB);
//         assert_eq!(d[7], 0x89);
//         assert_eq!(d[8], 0x54);
//         assert_eq!(d[9], 0x76);
//         assert_eq!(d[10], 0x10);
//         assert_eq!(d[11], 0x32);

//         unsafe {
//             let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
//                 as *mut GainSTMBodyInitial<[u16]>);
//             b.set_mode(Mode::PhaseFull);
//         }
//         assert_eq!(d[4], Mode::PhaseFull as _);

//         unsafe {
//             let b = &mut *(std::ptr::slice_from_raw_parts_mut(d.as_mut_ptr(), size)
//                 as *mut GainSTMBodyInitial<[u16]>);
//             b.set_mode(Mode::PhaseHalf);
//         }
//         assert_eq!(d[4], Mode::PhaseHalf as _);
//     }
// }
