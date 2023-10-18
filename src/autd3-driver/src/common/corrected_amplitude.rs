/*
 * File: corrected_amplitude.rs
 * Project: common
 * Created Date: 18/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{defined::float, derive::prelude::AUTDInternalError};

use super::Amplitude;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CorrectedAmplitude {
    value: float,
    alpha: float,
}

impl CorrectedAmplitude {
    pub const DEFAULT_ALPHA: float = 0.803;

    pub const MAX: CorrectedAmplitude = CorrectedAmplitude {
        value: 1.0,
        alpha: Self::DEFAULT_ALPHA,
    };
    pub const MIN: CorrectedAmplitude = CorrectedAmplitude {
        value: 0.0,
        alpha: Self::DEFAULT_ALPHA,
    };

    pub fn new(value: float) -> Result<Self, AUTDInternalError> {
        if !(0.0..=1.0).contains(&value) {
            Err(AUTDInternalError::AmplitudeOutOfRange(value))
        } else {
            Ok(Self {
                value,
                alpha: Self::DEFAULT_ALPHA,
            })
        }
    }

    pub fn new_clamped(value: float) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            alpha: Self::DEFAULT_ALPHA,
        }
    }

    pub fn with_alpha(self, alpha: float) -> Self {
        Self { alpha, ..self }
    }

    pub const fn value(&self) -> float {
        self.value
    }

    pub const fn alpha(&self) -> float {
        self.alpha
    }
}

impl From<CorrectedAmplitude> for Amplitude {
    fn from(f: CorrectedAmplitude) -> Self {
        Self::new_clamped(f.value.powf(1. / f.alpha))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrected_amplitude() {
        let amp = CorrectedAmplitude::new(0.0);
        assert!(amp.is_ok());
        assert_eq!(amp.unwrap().value(), 0.0);

        let amp = CorrectedAmplitude::new(1.0);
        assert!(amp.is_ok());
        assert_eq!(amp.unwrap().value(), 1.0);

        let amp = CorrectedAmplitude::new(1.1);
        assert!(amp.is_err());

        let amp = CorrectedAmplitude::new(-0.1);
        assert!(amp.is_err());
    }

    #[test]
    fn corrected_amplitude_with_alpha() {
        let amp = CorrectedAmplitude::new(1.0).unwrap().with_alpha(0.5);
        assert_eq!(amp.value(), 1.0);
        assert_eq!(amp.alpha(), 0.5);
    }

    #[test]
    fn corrected_amplitude_clamped() {
        let amp = CorrectedAmplitude::new_clamped(0.0);
        assert_eq!(amp.value(), 0.0);

        let amp = CorrectedAmplitude::new_clamped(1.0);
        assert_eq!(amp.value(), 1.0);

        let amp = CorrectedAmplitude::new_clamped(1.1);
        assert_eq!(amp.value(), 1.0);

        let amp = CorrectedAmplitude::new_clamped(-0.1);
        assert_eq!(amp.value(), 0.0);
    }

    #[test]
    fn corrected_amplitude_into_amplitude() {
        let value = 0.5;
        let amp: Amplitude = CorrectedAmplitude::new_clamped(value).into();

        assert_approx_eq::assert_approx_eq!(
            amp.value(),
            value.powf(1. / CorrectedAmplitude::DEFAULT_ALPHA)
        );
    }

    #[test]
    fn corrected_amplitude_clone() {
        let amp = CorrectedAmplitude::new_clamped(0.5);
        let amp2 = Clone::clone(&amp);
        assert_eq!(amp, amp2);
    }

    #[test]
    fn corrected_amplitude_debug() {
        let amp = CorrectedAmplitude::new_clamped(0.5);
        assert_eq!(
            format!("{:?}", amp),
            "CorrectedAmplitude { value: 0.5, alpha: 0.803 }"
        );
    }
}
