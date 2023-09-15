/*
 * File: fpga_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    cpu::RxMessage,
    defined::{float, Drive, METER, PI},
    derive::prelude::AUTDInternalError,
};

/// FPGA main clock frequency
pub const FPGA_CLK_FREQ: usize = 163840000;
pub const FPGA_SUB_CLK_FREQ_DIV: usize = 8;
/// FPGA sub clock frequency
pub const FPGA_SUB_CLK_FREQ: usize = FPGA_CLK_FREQ / FPGA_SUB_CLK_FREQ_DIV;

pub const FOCUS_STM_FIXED_NUM_UNIT: float = 0.025e-3 * METER;
pub const FOCUS_STM_FIXED_NUM_WIDTH: usize = 18;
pub const FOCUS_STM_FIXED_NUM_UPPER: i32 = (1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) - 1;
pub const FOCUS_STM_FIXED_NUM_LOWER: i32 = -(1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1));

pub const MAX_CYCLE: u16 = 8191;

pub const SAMPLING_FREQ_DIV_MIN: u32 = 4096 / FPGA_SUB_CLK_FREQ_DIV as u32;

pub const MOD_BUF_SIZE_MAX: usize = 65536;

pub const FOCUS_STM_BUF_SIZE_MAX: usize = 65536;
pub const GAIN_STM_BUF_SIZE_MAX: usize = 1024;
pub const GAIN_STM_LEGACY_BUF_SIZE_MAX: usize = 2048;

#[derive(Clone)]
#[repr(C)]
pub struct STMFocus {
    pub(crate) buf: [u16; 4],
}

impl STMFocus {
    pub fn set(
        &mut self,
        x: float,
        y: float,
        z: float,
        duty_shift: u8,
    ) -> Result<(), AUTDInternalError> {
        let ix = (x / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let iy = (y / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        let iz = (z / FOCUS_STM_FIXED_NUM_UNIT).round() as i32;
        if !(FOCUS_STM_FIXED_NUM_LOWER..=FOCUS_STM_FIXED_NUM_UPPER).contains(&ix)
            || !(FOCUS_STM_FIXED_NUM_LOWER..=FOCUS_STM_FIXED_NUM_UPPER).contains(&iy)
            || !(FOCUS_STM_FIXED_NUM_LOWER..=FOCUS_STM_FIXED_NUM_UPPER).contains(&iz)
        {
            return Err(AUTDInternalError::FocusSTMPointOutOfRange(x, y, z));
        }
        self.buf[0] = (ix & 0xFFFF) as u16;
        self.buf[1] = ((iy << 2) & 0xFFFC) as u16
            | ((ix >> 30) & 0x0002) as u16
            | ((ix >> 16) & 0x0001) as u16;
        self.buf[2] = ((iz << 4) & 0xFFF0) as u16
            | ((iy >> 28) & 0x0008) as u16
            | ((iy >> 14) & 0x0007) as u16;
        self.buf[3] = (((duty_shift as u16) << 6) & 0x3FC0)
            | ((iz >> 26) & 0x0020) as u16
            | ((iz >> 12) & 0x001F) as u16;
        Ok(())
    }
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

#[derive(Debug)]
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

#[derive(Debug)]
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

/// FPGA information
#[repr(C)]
pub struct FPGAInfo {
    info: u8,
}

impl FPGAInfo {
    pub const fn new(info: u8) -> Self {
        Self { info }
    }

    /// Check if thermal sensor is asserted
    pub const fn is_thermal_assert(&self) -> bool {
        (self.info & 0x01) != 0
    }

    pub const fn info(&self) -> u8 {
        self.info
    }
}

impl From<&RxMessage> for FPGAInfo {
    fn from(msg: &RxMessage) -> Self {
        Self { info: msg.data }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;
    use crate::defined::PI;

    #[test]
    fn stm_focus() {
        let mut p = STMFocus { buf: [0; 4] };

        let x = FOCUS_STM_FIXED_NUM_UNIT;
        let y = 2. * FOCUS_STM_FIXED_NUM_UNIT;
        let z = 3. * FOCUS_STM_FIXED_NUM_UNIT;
        let duty_shift = 4;

        assert!(p.set(x, y, z, duty_shift).is_ok());

        assert_eq!((p.buf[0] as u32) & ((1 << FOCUS_STM_FIXED_NUM_WIDTH) - 1), 1);
        assert_eq!(
            ((p.buf[1] >> 2) as u32) & ((1 << FOCUS_STM_FIXED_NUM_WIDTH) - 1),
            2
        );
        assert_eq!(
            ((p.buf[2] >> 4) as u32) & ((1 << FOCUS_STM_FIXED_NUM_WIDTH) - 1),
            3
        );
        assert_eq!((p.buf[3] >> 6) & 0xFF, 4);

        let x = -FOCUS_STM_FIXED_NUM_UNIT;
        let y = -2. * FOCUS_STM_FIXED_NUM_UNIT;
        let z = -3. * FOCUS_STM_FIXED_NUM_UNIT;
        let duty_shift = 0xFF;

        assert!(p.set(x, y, z, duty_shift).is_ok());

        assert_eq!(p.buf[0], 0xFFFF);
        assert_eq!(p.buf[1] & 0b01, 0b01);
        assert_eq!(p.buf[1] & 0b10, 0b10);

        assert_eq!(p.buf[1] & 0b1111111111111100, 0b1111111111111000);
        assert_eq!(p.buf[2] & 0b0111, 0b0111);
        assert_eq!(p.buf[2] & 0b1000, 0b1000);

        assert_eq!(p.buf[2] & 0b1111111111110000, 0b1111111111010000);
        assert_eq!(p.buf[3] & 0b011111, 0b011111);
        assert_eq!(p.buf[3] & 0b100000, 0b100000);

        assert_eq!((p.buf[3] >> 6) & 0xFF, 0xFF);

        let x = FOCUS_STM_FIXED_NUM_UNIT * ((1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) - 1) as float;
        let y = FOCUS_STM_FIXED_NUM_UNIT * ((1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) - 1) as float;
        let z = FOCUS_STM_FIXED_NUM_UNIT * ((1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) - 1) as float;
        let duty_shift = 0;

        assert!(p.set(x, y, z, duty_shift).is_ok());

        assert!(p
            .set(x + FOCUS_STM_FIXED_NUM_UNIT, y, z, duty_shift)
            .is_err());
        assert!(p
            .set(x, y + FOCUS_STM_FIXED_NUM_UNIT, z, duty_shift)
            .is_err());
        assert!(p
            .set(x, y, z + FOCUS_STM_FIXED_NUM_UNIT, duty_shift)
            .is_err());

        let x = -FOCUS_STM_FIXED_NUM_UNIT * (1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) as float;
        let y = -FOCUS_STM_FIXED_NUM_UNIT * (1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) as float;
        let z = -FOCUS_STM_FIXED_NUM_UNIT * (1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) as float;
        let duty_shift = 0;

        assert!(p.set(x, y, z, duty_shift).is_ok());

        assert!(p
            .set(x - FOCUS_STM_FIXED_NUM_UNIT, y, z, duty_shift)
            .is_err());
        assert!(p
            .set(x, y - FOCUS_STM_FIXED_NUM_UNIT, z, duty_shift)
            .is_err());
        assert!(p
            .set(x, y, z - FOCUS_STM_FIXED_NUM_UNIT, duty_shift)
            .is_err());
    }

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
        dbg!(d);

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

        let d = AdvancedDrivePhase { phase: 0x0001 };
        let dc = d;
        assert_eq!(d.phase, dc.phase);
        dbg!(d);

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

        let d = AdvancedDriveDuty { duty: 0x0001 };
        let dc = d;
        assert_eq!(d.duty, dc.duty);
        dbg!(d);

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

    #[test]
    fn fpga_info() {
        assert_eq!(size_of::<FPGAInfo>(), 1);

        let info = FPGAInfo::new(0x00);
        assert!(!info.is_thermal_assert());
        assert_eq!(info.info(), 0x00);

        let info = FPGAInfo::new(0x01);
        assert!(info.is_thermal_assert());
        assert_eq!(info.info(), 0x01);

        let info = FPGAInfo::new(0x02);
        assert!(!info.is_thermal_assert());
        assert_eq!(info.info(), 0x02);

        let rx = RxMessage { ack: 0, data: 0x01 };
        let info = FPGAInfo::from(&rx);
        assert!(info.is_thermal_assert());
        assert_eq!(info.info(), 0x01);
    }
}
