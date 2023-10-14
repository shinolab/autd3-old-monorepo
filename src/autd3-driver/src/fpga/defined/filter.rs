/*
 * File: filter.rs
 * Project: defined
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::defined::{float, PI};

#[repr(C)]
pub struct FilterPhase {
    pub phase: i16,
}

impl FilterPhase {
    pub fn to_phase(d: float, cycle: u16) -> i16 {
        (((d / (2.0 * PI) * cycle as float).round() as i32) % (cycle as i32)) as _
    }

    pub fn set(&mut self, d: float, cycle: u16) {
        self.phase = Self::to_phase(d, cycle);
    }
}

#[repr(C)]
pub struct FilterDuty {
    pub duty: i16,
}

impl FilterDuty {
    pub fn to_duty(d: float, cycle: u16) -> i16 {
        (cycle as float * d.clamp(-1., 1.).asin() / PI).round() as _
    }

    pub fn set(&mut self, d: float, cycle: u16) {
        self.duty = Self::to_duty(d, cycle);
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;
    use crate::defined::PI;

    #[test]
    fn phase_filter() {
        assert_eq!(size_of::<FilterPhase>(), 2);

        let mut d = 0x0000i16;

        let cycle = 4096;
        unsafe {
            (*(&mut d as *mut _ as *mut FilterPhase)).set(0.0, cycle);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(PI, cycle);
            assert_eq!(d, 2048);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(1.5 * PI, cycle);
            assert_eq!(d, 2048 + 1024);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(2.0 * PI, cycle);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(3.0 * PI, cycle);
            assert_eq!(d, 2048);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-PI, cycle);
            assert_eq!(d, -2048);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-1.5 * PI, cycle);
            assert_eq!(d, -2048 - 1024);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-2.0 * PI, cycle);
            assert_eq!(d, 0);
        }

        let cycle = 2000;
        unsafe {
            (*(&mut d as *mut _ as *mut FilterPhase)).set(0.0, cycle);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(PI, cycle);
            assert_eq!(d, 1000);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(1.5 * PI, cycle);
            assert_eq!(d, 1000 + 500);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(2.0 * PI, cycle);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(3.0 * PI, cycle);
            assert_eq!(d, 1000);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-PI, cycle);
            assert_eq!(d, -1000);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-1.5 * PI, cycle);
            assert_eq!(d, -1000 - 500);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-2.0 * PI, cycle);
            assert_eq!(d, 0);
        }
    }

    #[test]
    fn duty_filter() {
        assert_eq!(size_of::<FilterDuty>(), 2);

        let mut d = 0x0000i16;

        let cycle = 4096;
        unsafe {
            (*(&mut d as *mut _ as *mut FilterDuty)).set(0.0, cycle);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(0.5, cycle);
            assert_eq!(d, 683);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(1.0, cycle);
            assert_eq!(d, 2048);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(1.5, cycle);
            assert_eq!(d, 2048);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(-0.5, cycle);
            assert_eq!(d, -683);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(-1.0, cycle);
            assert_eq!(d, -2048);
        }

        let cycle = 2000;
        unsafe {
            (*(&mut d as *mut _ as *mut FilterDuty)).set(0.0, cycle);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(0.5, cycle);
            assert_eq!(d, 333);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(1.0, cycle);
            assert_eq!(d, 1000);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(1.5, cycle);
            assert_eq!(d, 1000);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(-0.5, cycle);
            assert_eq!(d, -333);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(-1.0, cycle);
            assert_eq!(d, -1000);
        }
    }
}
