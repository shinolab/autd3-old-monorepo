/*
 * File: filter.rs
 * Project: defined
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
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
    pub fn to_phase(d: float) -> i16 {
        (((d / (2.0 * PI) * 512.0).round() as i32) % 512) as _
    }

    pub fn set(&mut self, d: float) {
        self.phase = Self::to_phase(d);
    }
}

#[repr(C)]
pub struct FilterDuty {
    pub duty: i16,
}

impl FilterDuty {
    pub fn to_duty(d: float) -> i16 {
        (512.0 * d.clamp(-1., 1.).asin() / PI).round() as _
    }

    pub fn set(&mut self, d: float) {
        self.duty = Self::to_duty(d);
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

        unsafe {
            (*(&mut d as *mut _ as *mut FilterPhase)).set(0.0);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(PI);
            assert_eq!(d, 256);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(1.5 * PI);
            assert_eq!(d, 256 + 128);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(2.0 * PI);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(3.0 * PI);
            assert_eq!(d, 256);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-PI);
            assert_eq!(d, -256);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-1.5 * PI);
            assert_eq!(d, -256 - 128);

            (*(&mut d as *mut _ as *mut FilterPhase)).set(-2.0 * PI);
            assert_eq!(d, 0);
        }
    }

    #[test]
    fn duty_filter() {
        assert_eq!(size_of::<FilterDuty>(), 2);

        let mut d = 0x0000i16;

        unsafe {
            (*(&mut d as *mut _ as *mut FilterDuty)).set(0.0);
            assert_eq!(d, 0);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(0.5);
            assert_eq!(d, 85);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(1.0);
            assert_eq!(d, 256);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(1.5);
            assert_eq!(d, 256);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(-0.5);
            assert_eq!(d, -85);

            (*(&mut d as *mut _ as *mut FilterDuty)).set(-1.0);
            assert_eq!(d, -256);
        }
    }
}
