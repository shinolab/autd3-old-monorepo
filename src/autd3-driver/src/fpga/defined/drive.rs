/*
 * File: drive.rs
 * Project: defined
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{common::Drive, defined::PI};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FPGADrive {
    pub phase: u8,
    pub intensity: u8,
}

impl FPGADrive {
    pub fn to_phase(d: &Drive) -> u8 {
        (((d.phase / (2.0 * PI) * 256.0).round() as i32) & 0xFF) as _
    }

    pub fn to_intensity(d: &Drive) -> u8 {
        d.intensity.value()
    }

    pub fn set(&mut self, d: &Drive) {
        self.intensity = Self::to_intensity(d);
        self.phase = Self::to_phase(d);
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;
    use crate::{common::EmitIntensity, defined::PI};

    #[test]
    fn drive() {
        assert_eq!(size_of::<FPGADrive>(), 2);

        let d = FPGADrive {
            phase: 0x01,
            intensity: 0x02,
        };
        let dc = Clone::clone(&d);
        assert_eq!(d.phase, dc.phase);
        assert_eq!(d.intensity, dc.intensity);

        let mut d = [0x00u8; 2];

        unsafe {
            let s = Drive {
                phase: 0.0,
                intensity: EmitIntensity::MIN,
            };
            (*(&mut d as *mut _ as *mut FPGADrive)).set(&s);
            assert_eq!(d[0], 0x00);
            assert_eq!(d[1], 0x00);

            let s = Drive {
                phase: PI,
                intensity: EmitIntensity::new(84),
            };
            (*(&mut d as *mut _ as *mut FPGADrive)).set(&s);
            assert_eq!(d[0], 128);
            assert_eq!(d[1], 84);

            let s = Drive {
                phase: 2.0 * PI,
                intensity: EmitIntensity::MAX,
            };
            (*(&mut d as *mut _ as *mut FPGADrive)).set(&s);
            assert_eq!(d[0], 0x00);
            assert_eq!(d[1], 0xFF);

            let s = Drive {
                phase: 3.0 * PI,
                intensity: EmitIntensity::MAX,
            };
            (*(&mut d as *mut _ as *mut FPGADrive)).set(&s);
            assert_eq!(d[0], 128);
            assert_eq!(d[1], 0xFF);

            let s = Drive {
                phase: -PI,
                intensity: EmitIntensity::MIN,
            };
            (*(&mut d as *mut _ as *mut FPGADrive)).set(&s);
            assert_eq!(d[0], 128);
            assert_eq!(d[1], 0);
        }
    }
}
