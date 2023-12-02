/*
 * File: emit_intensity.rs
 * Project: common
 * Created Date: 11/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::defined::{float, PI};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct EmitIntensity {
    value: u8,
}

impl EmitIntensity {
    pub const MAX: EmitIntensity = EmitIntensity { value: 255 };
    pub const MIN: EmitIntensity = EmitIntensity { value: 0 };
    pub const DEFAULT_CORRECTED_ALPHA: float = 0.803;

    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn with_correction(value: u8) -> Self {
        Self::with_correction_alpha(value, Self::DEFAULT_CORRECTED_ALPHA)
    }

    pub fn with_correction_alpha(value: u8, alpha: float) -> Self {
        Self {
            value: ((value as float / 255.).powf(1. / alpha).asin() / PI * 510.0).round() as u8,
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

impl From<u8> for EmitIntensity {
    fn from(v: u8) -> Self {
        Self::new(v)
    }
}

impl std::ops::Div<u8> for EmitIntensity {
    type Output = Self;

    fn div(self, rhs: u8) -> Self::Output {
        Self::new(self.value / rhs)
    }
}

impl std::ops::Add<EmitIntensity> for EmitIntensity {
    type Output = Self;

    fn add(self, rhs: EmitIntensity) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl std::ops::Sub<EmitIntensity> for EmitIntensity {
    type Output = Self;

    fn sub(self, rhs: EmitIntensity) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn test_new() {
        for i in 0x00..=0xFF {
            let intensity = EmitIntensity::new(i);
            assert_eq!(intensity.value(), i);
        }
    }

    #[test]
    fn test_with_correction() {
        for i in 0..=0xFF {
            let intensity = EmitIntensity::with_correction(i);
            assert_eq!(
                intensity.value(),
                ((i as float / 255.)
                    .powf(1. / EmitIntensity::DEFAULT_CORRECTED_ALPHA)
                    .asin()
                    / PI
                    * 510.0)
                    .round() as u8
            );
        }
    }

    #[test]
    fn test_div() {
        let intensity = EmitIntensity::new(0xFF);
        assert_eq!(intensity / 2, EmitIntensity::new(0x7F));
    }

    #[test]
    fn test_add() {
        let intensity1 = EmitIntensity::new(0x01);
        let intensity2 = EmitIntensity::new(0x02);
        assert_eq!(intensity1 + intensity2, EmitIntensity::new(0x03));
    }

    #[test]
    fn test_sub() {
        let intensity1 = EmitIntensity::new(0x03);
        let intensity2 = EmitIntensity::new(0x01);
        assert_eq!(intensity1 - intensity2, EmitIntensity::new(0x02));
    }

    #[test]
    fn test_clone() {
        let intensity = EmitIntensity::new(0);
        assert_eq!(intensity.value(), intensity.clone().value());
    }

    #[test]
    fn test_debug() {
        let intensity = EmitIntensity::new(0);
        assert_eq!(format!("{:?}", intensity), "EmitIntensity { value: 0 }");
    }

    #[test]
    fn test_ord() {
        let intensity1 = EmitIntensity::new(0);
        let intensity2 = EmitIntensity::new(1);
        assert!(intensity1 < intensity2);
        assert_eq!(intensity1.min(intensity2), intensity1);
    }

    #[test]
    fn hash() {
        let intensity = EmitIntensity::new(0);
        let mut s = DefaultHasher::new();
        assert_eq!(intensity.hash(&mut s), 0.hash(&mut s));
        s.finish();
    }
}
