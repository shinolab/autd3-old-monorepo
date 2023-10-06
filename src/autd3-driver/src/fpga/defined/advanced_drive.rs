/*
 * File: advanced_drive.rs
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

use crate::defined::{float, Drive, PI};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;
    use crate::defined::PI;

    #[test]
    fn phase() {
        assert_eq!(size_of::<AdvancedDrivePhase>(), 2);

        let d = AdvancedDrivePhase { phase: 0x0001 };
        let dc = d;
        assert_eq!(d.phase, dc.phase);

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
}
