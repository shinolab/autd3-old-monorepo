/*
 * File: phase.rs
 * Project: common
 * Created Date: 02/12/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::defined::{float, PI};

pub struct Rad;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase {
    value: u8,
}

impl Phase {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_rad(phase: float) -> Self {
        Self {
            value: (((phase / (2.0 * PI) * 256.0).round() as i32) & 0xFF) as _,
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn radian(&self) -> float {
        self.value as float / 256.0 * 2.0 * PI
    }
}

impl From<u8> for Phase {
    fn from(v: u8) -> Self {
        Self::new(v)
    }
}

impl std::ops::Mul<Rad> for float {
    type Output = Phase;

    fn mul(self, _rhs: Rad) -> Self::Output {
        Self::Output::from_rad(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::defined::PI;

    #[test]
    fn phase_clone() {
        let a = PI * Rad;
        let b = a.clone();
        assert_eq!(a.value, b.value);
    }

    #[test]
    fn phase_from() {
        assert_eq!(Phase::from(0x90).value, 0x90);

        assert_eq!(Phase::from_rad(0.).value, 0);
        assert_eq!(Phase::from_rad(PI).value, 128);
        assert_eq!(Phase::from_rad(PI * 510.0 / 256.0).value, 255);
        assert_eq!(Phase::from_rad(2. * PI).value, 0);
    }

    #[test]
    fn phase_radian() {
        assert_eq!(Phase::from_rad(0.).radian(), 0.);
        assert_eq!(Phase::from_rad(PI).radian(), PI);
        assert_eq!(
            Phase::from_rad(PI * 510.0 / 256.0).radian(),
            PI * 510.0 / 256.0
        );
        assert_eq!(Phase::from_rad(2. * PI).radian(), 0.);
    }

    #[test]
    fn phase_debug() {
        let a = Phase::from_rad(PI);
        assert_eq!(format!("{:?}", a), "Phase { value: 128 }");
    }
}
