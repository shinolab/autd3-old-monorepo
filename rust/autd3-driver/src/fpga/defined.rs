/*
 * File: fpga_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{float, METER, PI};

pub const FPGA_CLK_FREQ: usize = 163840000;
pub const FPGA_SUB_CLK_FREQ_DIV: usize = 8;
pub const FPGA_SUB_CLK_FREQ: usize = FPGA_CLK_FREQ / FPGA_SUB_CLK_FREQ_DIV;

pub const FOCUS_STM_FIXED_NUM_UNIT: float = 0.025e-3 * METER;

#[derive(Clone, Copy, Debug)]
pub struct Drive {
    pub phase: float,
    pub amp: float,
}

#[derive(Clone, Copy, Debug)]
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
        (510.0 * d.amp.clamp(0., 1.).asin() / PI).round() as _
    }

    pub fn set(&mut self, d: &Drive) {
        self.duty = Self::to_duty(d);
        self.phase = Self::to_phase(d);
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct AdvancedDrivePhase {
    pub phase: u16,
}

impl AdvancedDrivePhase {
    pub fn to_phase(d: &Drive, cycle: u16) -> u16 {
        ((d.phase / (2.0 * PI) * cycle as float).round() as i32).rem_euclid(cycle as i32) as _
    }

    pub fn set(&mut self, d: &Drive, cycle: u16) {
        self.phase = Self::to_phase(d, cycle);
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct AdvancedDriveDuty {
    pub duty: u16,
}

impl AdvancedDriveDuty {
    pub fn to_duty(d: &Drive, cycle: u16) -> u16 {
        (cycle as float * d.amp.clamp(0., 1.).asin() / PI).round() as _
    }

    pub fn set(&mut self, d: &Drive, cycle: u16) {
        self.duty = Self::to_duty(d, cycle);
    }
}

#[derive(Default)]
#[repr(C)]
pub struct FPGAInfo {
    info: u8,
}

impl FPGAInfo {
    pub fn is_thermal_assert(&self) -> bool {
        (self.info & 0x01) != 0
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;
    use crate::PI;

    #[test]
    fn legacy_drive() {
        assert_eq!(size_of::<LegacyDrive>(), 2);

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
            assert_eq!(d[1], 85);

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

    #[test]
    fn phase() {
        assert_eq!(size_of::<AdvancedDrivePhase>(), 2);

        let mut d = 0x0000u16;

        let cycle = 4096;
        unsafe {
            let s = Drive {
                phase: 0.0,
                amp: 0.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 0);

            let s = Drive {
                phase: PI,
                amp: 0.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 2048);

            let s = Drive {
                phase: 2.0 * PI,
                amp: 0.,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 0);

            let s = Drive {
                phase: 3.0 * PI,
                amp: 0.,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 2048);

            let s = Drive {
                phase: -PI,
                amp: 0.,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 2048);
        }

        let cycle = 2000;
        unsafe {
            let s = Drive {
                phase: 0.0,
                amp: 0.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 0);

            let s = Drive {
                phase: PI,
                amp: 0.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 1000);

            let s = Drive {
                phase: 2.0 * PI,
                amp: 0.,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 0);

            let s = Drive {
                phase: 3.0 * PI,
                amp: 0.,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 1000);

            let s = Drive {
                phase: -PI,
                amp: 0.,
            };
            (*(&mut d as *mut _ as *mut AdvancedDrivePhase)).set(&s, cycle);
            assert_eq!(d, 1000);
        }
    }

    #[test]
    fn duty() {
        assert_eq!(size_of::<AdvancedDriveDuty>(), 2);

        let mut d = 0x0000u16;

        let cycle = 4096;
        unsafe {
            let s = Drive {
                phase: 0.0,
                amp: 0.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 0);

            let s = Drive {
                phase: 0.0,
                amp: 0.5,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 683);

            let s = Drive {
                phase: 0.0,
                amp: 1.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 2048);

            let s = Drive {
                phase: 0.0,
                amp: 1.5,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 2048);

            let s = Drive {
                phase: 0.0,
                amp: -1.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 0);
        }

        let cycle = 2000;
        unsafe {
            let s = Drive {
                phase: 0.0,
                amp: 0.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 0);

            let s = Drive {
                phase: 0.0,
                amp: 0.5,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 333);

            let s = Drive {
                phase: 0.0,
                amp: 1.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 1000);

            let s = Drive {
                phase: 0.0,
                amp: 1.5,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 1000);

            let s = Drive {
                phase: 0.0,
                amp: -1.0,
            };
            (*(&mut d as *mut _ as *mut AdvancedDriveDuty)).set(&s, cycle);
            assert_eq!(d, 0);
        }
    }

    #[test]
    fn fpga_info() {
        assert_eq!(size_of::<FPGAInfo>(), 1);

        let info = FPGAInfo { info: 0x00 };
        assert!(!info.is_thermal_assert());

        let info = FPGAInfo { info: 0x01 };
        assert!(info.is_thermal_assert());

        let info = FPGAInfo { info: 0x02 };
        assert!(!info.is_thermal_assert());
    }
}
