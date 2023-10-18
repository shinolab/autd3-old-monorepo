/*
 * File: duty_ratio.rs
 * Project: common
 * Created Date: 14/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    defined::{float, PI},
    derive::prelude::AUTDInternalError,
};

use super::Amplitude;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DutyRatio {
    value: float,
}

impl DutyRatio {
    pub const MAX: DutyRatio = DutyRatio { value: 0.5 };
    pub const MIN: DutyRatio = DutyRatio { value: 0.0 };

    pub fn new(value: float) -> Result<Self, AUTDInternalError> {
        if !(0.0..=0.5).contains(&value) {
            Err(AUTDInternalError::DutyRatioOutOfRange(value))
        } else {
            Ok(Self { value })
        }
    }

    pub fn new_clamped(value: float) -> Self {
        Self {
            value: value.clamp(0.0, 0.5),
        }
    }

    pub const fn value(&self) -> float {
        self.value
    }
}

impl From<Amplitude> for DutyRatio {
    fn from(f: Amplitude) -> Self {
        Self {
            value: f.value().asin() / PI,
        }
    }
}

impl From<DutyRatio> for Amplitude {
    fn from(f: DutyRatio) -> Self {
        Self::new_clamped((f.value() * PI).sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duty_ratio() {
        let d = DutyRatio::new(0.0);
        assert!(d.is_ok());
        assert_eq!(d.unwrap().value(), 0.0);

        let d = DutyRatio::new(0.5);
        assert!(d.is_ok());
        assert_eq!(d.unwrap().value(), 0.5);

        let d = DutyRatio::new(0.6);
        assert!(d.is_err());

        let d = DutyRatio::new(-0.1);
        assert!(d.is_err());
    }

    #[test]
    fn duty_ratio_clamped() {
        let d = DutyRatio::new_clamped(0.0);
        assert_eq!(d.value(), 0.0);

        let d = DutyRatio::new_clamped(0.5);
        assert_eq!(d.value(), 0.5);

        let d = DutyRatio::new_clamped(0.6);
        assert_eq!(d.value(), 0.5);

        let d = DutyRatio::new_clamped(-0.1);
        assert_eq!(d.value(), 0.0);
    }

    #[test]
    fn duty_ratio_from_amplitude() {
        let duty = DutyRatio::from(Amplitude::MIN);
        assert_eq!(duty, DutyRatio::MIN);

        let duty = DutyRatio::from(Amplitude::MAX);
        assert_eq!(duty, DutyRatio::MAX);

        let amp = Amplitude::new_clamped(0.3090169943749474);
        let duty = DutyRatio::from(amp);
        assert_approx_eq::assert_approx_eq!(duty.value(), 0.1);
    }

    #[test]
    fn amplitude_from_duty_ratio() {
        let amp = Amplitude::from(DutyRatio::MIN);
        assert_eq!(amp, Amplitude::MIN);

        let amp = Amplitude::from(DutyRatio::MAX);
        assert_eq!(amp, Amplitude::MAX);

        let amp = Amplitude::from(DutyRatio::new_clamped(0.1));
        assert_approx_eq::assert_approx_eq!(amp.value(), 0.3090169943749474);
    }

    #[test]
    fn duty_ratio_clone() {
        let duty = DutyRatio::new_clamped(0.2);
        let duty2 = Clone::clone(&duty);
        assert_eq!(duty, duty2);
    }

    #[test]
    fn duty_ratio_debug() {
        let duty = DutyRatio::new_clamped(0.2);
        assert_eq!(format!("{:?}", duty), "DutyRatio { value: 0.2 }");
    }
}
