/*
 * File: amplitude.rs
 * Project: common
 * Created Date: 14/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    defined::{float, PI},
    derive::prelude::AUTDInternalError,
};

use super::DutyRatio;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Amplitude {
    value: float,
}

impl Amplitude {
    pub const MAX: Amplitude = Amplitude { value: 1.0 };
    pub const MIN: Amplitude = Amplitude { value: 0.0 };

    pub fn new(value: float) -> Result<Self, AUTDInternalError> {
        if !(0.0..=1.0).contains(&value) {
            Err(AUTDInternalError::AmplitudeOutOfRange(value))
        } else {
            Ok(Self { value })
        }
    }

    pub fn new_clamped(value: float) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    pub const fn value(&self) -> float {
        self.value
    }
}

impl From<float> for Amplitude {
    fn from(value: float) -> Self {
        Amplitude::new_clamped(value)
    }
}

impl From<DutyRatio> for Amplitude {
    fn from(f: DutyRatio) -> Self {
        Self {
            value: (f.value() * PI).sin(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amplitude() {
        let amp = Amplitude::new(0.0);
        assert!(amp.is_ok());
        assert_eq!(amp.unwrap().value(), 0.0);

        let amp = Amplitude::new(1.0);
        assert!(amp.is_ok());
        assert_eq!(amp.unwrap().value(), 1.0);

        let amp = Amplitude::new(1.1);
        assert!(amp.is_err());

        let amp = Amplitude::new(-0.1);
        assert!(amp.is_err());
    }

    #[test]
    fn amplitude_clamped() {
        let amp = Amplitude::new_clamped(0.0);
        assert_eq!(amp.value(), 0.0);

        let amp = Amplitude::new_clamped(1.0);
        assert_eq!(amp.value(), 1.0);

        let amp = Amplitude::new_clamped(1.1);
        assert_eq!(amp.value(), 1.0);

        let amp = Amplitude::new_clamped(-0.1);
        assert_eq!(amp.value(), 0.0);
    }

    #[test]
    fn amplitude_from_f64() {
        let amp = Amplitude::from(0.0);
        assert_eq!(amp.value(), 0.0);

        let amp = Amplitude::from(1.0);
        assert_eq!(amp.value(), 1.0);

        let amp = Amplitude::from(1.1);
        assert_eq!(amp.value(), 1.0);

        let amp = Amplitude::from(-0.1);
        assert_eq!(amp.value(), 0.0);
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
    fn amplitude_clone() {
        let amp = Amplitude::new_clamped(0.5);
        let amp2 = Clone::clone(&amp);
        assert_eq!(amp, amp2);
    }

    #[test]
    fn amplitude_debug() {
        let amp = Amplitude::new_clamped(0.5);
        assert_eq!(format!("{:?}", amp), "Amplitude { value: 0.5 }");
    }
}
