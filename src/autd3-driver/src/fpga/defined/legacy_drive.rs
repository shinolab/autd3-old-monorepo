/*
 * File: legacy_drive.rs
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

use crate::defined::{Drive, PI};

#[derive(Clone, Copy)]
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
        (512.0 * d.amp.clamp(0.00613588464, 1.).asin() / PI - 1.0).round() as _
    }

    pub fn set(&mut self, d: &Drive) {
        self.duty = Self::to_duty(d);
        self.phase = Self::to_phase(d);
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;
    use crate::defined::PI;

    #[test]
    fn legacy_drive() {
        assert_eq!(size_of::<LegacyDrive>(), 2);

        let d = LegacyDrive {
            phase: 0x01,
            duty: 0x02,
        };
        let dc = d;
        assert_eq!(d.phase, dc.phase);
        assert_eq!(d.duty, dc.duty);

        let mut d = [0x00u8; 2];

        unsafe {
            let s = Drive {
                phase: 0.0,
                amp: 0.0,
            };
            (*(&mut d as *mut _ as *mut LegacyDrive)).set(&s);
            assert_eq!(d[0], 0x00);
            assert_eq!(d[1], 0x00);

            let s = Drive {
                phase: PI,
                amp: 0.5,
            };
            (*(&mut d as *mut _ as *mut LegacyDrive)).set(&s);
            assert_eq!(d[0], 128);
            assert_eq!(d[1], 84);

            let s = Drive {
                phase: 2.0 * PI,
                amp: 1.0,
            };
            (*(&mut d as *mut _ as *mut LegacyDrive)).set(&s);
            assert_eq!(d[0], 0x00);
            assert_eq!(d[1], 0xFF);

            let s = Drive {
                phase: 3.0 * PI,
                amp: 1.5,
            };
            (*(&mut d as *mut _ as *mut LegacyDrive)).set(&s);
            assert_eq!(d[0], 128);
            assert_eq!(d[1], 0xFF);

            let s = Drive {
                phase: -PI,
                amp: -1.0,
            };
            (*(&mut d as *mut _ as *mut LegacyDrive)).set(&s);
            assert_eq!(d[0], 128);
            assert_eq!(d[1], 0);
        }
    }
}
