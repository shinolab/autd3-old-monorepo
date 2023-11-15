/*
 * File: emit_intensity.rs
 * Project: common
 * Created Date: 11/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    defined::{float, PI},
    derive::prelude::AUTDInternalError,
};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EmitIntensity {
    pulse_width: u16,
}

impl EmitIntensity {
    pub const MAX: EmitIntensity = EmitIntensity { pulse_width: 256 };
    pub const MIN: EmitIntensity = EmitIntensity { pulse_width: 0 };
    pub const DEFAULT_CORRECTED_ALPHA: float = 0.803;

    pub fn normalized(&self) -> float {
        (self.duty_ratio() * PI).sin()
    }

    pub fn duty_ratio(&self) -> float {
        self.pulse_width as float / 512.0
    }

    pub fn pulse_width(&self) -> u16 {
        self.pulse_width
    }

    pub fn new_normalized(value: float) -> Result<Self, AUTDInternalError> {
        if !(0.0..=1.0).contains(&value) {
            Err(AUTDInternalError::AmplitudeOutOfRange(value))
        } else {
            match (value.asin() / PI * 512.0).round() as u16 {
                1 => Ok(EmitIntensity { pulse_width: 0 }),
                v => Ok(EmitIntensity { pulse_width: v }),
            }
        }
    }

    pub fn new_normalized_clamped(value: float) -> Self {
        Self::new_normalized(value.clamp(0.0, 1.0)).unwrap()
    }

    pub fn new_normalized_corrected(value: float) -> Result<Self, AUTDInternalError> {
        Self::new_normalized_corrected_with_alpha(value, Self::DEFAULT_CORRECTED_ALPHA)
    }

    pub fn new_normalized_corrected_with_alpha(
        value: float,
        alpha: float,
    ) -> Result<Self, AUTDInternalError> {
        if !(0.0..=1.0).contains(&value) {
            Err(AUTDInternalError::AmplitudeOutOfRange(value))
        } else {
            match (value.powf(1. / alpha).asin() / PI * 512.0).round() as u16 {
                1 => Ok(EmitIntensity { pulse_width: 0 }),
                v => Ok(EmitIntensity { pulse_width: v }),
            }
        }
    }

    pub fn new_duty_ratio(value: float) -> Result<Self, AUTDInternalError> {
        if !(0.0..=0.5).contains(&value) {
            Err(AUTDInternalError::DutyRatioOutOfRange(value))
        } else {
            match (value * 512.0).round() as u16 {
                1 => Ok(EmitIntensity { pulse_width: 0 }),
                v => Ok(EmitIntensity { pulse_width: v }),
            }
        }
    }

    pub fn new_pulse_width(value: u16) -> Result<Self, AUTDInternalError> {
        if value == 1 || !(0..=256).contains(&value) {
            Err(AUTDInternalError::PulseWidthOutOfRange(value))
        } else {
            Ok(EmitIntensity { pulse_width: value })
        }
    }
}

pub trait TryIntoEmitIntensity {
    fn try_into(self) -> Result<EmitIntensity, AUTDInternalError>;
}

impl TryIntoEmitIntensity for float {
    fn try_into(self) -> Result<EmitIntensity, AUTDInternalError> {
        EmitIntensity::new_normalized(self)
    }
}

impl TryIntoEmitIntensity for u16 {
    fn try_into(self) -> Result<EmitIntensity, AUTDInternalError> {
        EmitIntensity::new_pulse_width(self)
    }
}

impl TryIntoEmitIntensity for EmitIntensity {
    fn try_into(self) -> Result<EmitIntensity, AUTDInternalError> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_normalized() {
        for i in 0..=256 {
            let intensity = TryIntoEmitIntensity::try_into((i as float / 512.0 * PI).sin());
            assert!(intensity.is_ok());
            let intensity = intensity.unwrap();
            if i == 1 {
                assert_eq!(intensity.pulse_width(), 0);
            } else {
                assert_eq!(intensity.pulse_width(), i);
            }
        }

        let intensity = EmitIntensity::new_normalized(-0.1);
        assert_eq!(
            intensity.unwrap_err(),
            AUTDInternalError::AmplitudeOutOfRange(-0.1)
        );

        let intensity = EmitIntensity::new_normalized(1.1);
        assert_eq!(
            intensity.unwrap_err(),
            AUTDInternalError::AmplitudeOutOfRange(1.1)
        );

        let intensity = EmitIntensity::new_normalized_clamped(-0.1);
        assert_eq!(intensity.pulse_width(), 0);

        let intensity = EmitIntensity::new_normalized_clamped(1.1);
        assert_eq!(intensity.pulse_width(), 256);
    }

    #[test]
    fn test_new_normalized_corrected() {
        for i in 0..=256 {
            let intensity = EmitIntensity::new_normalized_corrected(
                (i as float / 512.0 * PI)
                    .sin()
                    .powf(EmitIntensity::DEFAULT_CORRECTED_ALPHA),
            );
            assert!(intensity.is_ok());
            let intensity = intensity.unwrap();
            if i == 1 {
                assert_eq!(intensity.pulse_width(), 0);
            } else {
                assert_eq!(intensity.pulse_width(), i);
            }
        }

        let intensity = EmitIntensity::new_normalized_corrected(-0.1);
        assert_eq!(
            intensity.unwrap_err(),
            AUTDInternalError::AmplitudeOutOfRange(-0.1)
        );

        let intensity = EmitIntensity::new_normalized_corrected(1.1);
        assert_eq!(
            intensity.unwrap_err(),
            AUTDInternalError::AmplitudeOutOfRange(1.1)
        );
    }

    #[test]
    fn test_new_duty_ratio() {
        for i in 0..=256 {
            let intensity = EmitIntensity::new_duty_ratio(i as float / 512.0);
            assert!(intensity.is_ok());
            let intensity = intensity.unwrap();
            if i == 1 {
                assert_eq!(intensity.pulse_width(), 0);
            } else {
                assert_eq!(intensity.pulse_width(), i);
            }
        }

        let intensity = EmitIntensity::new_duty_ratio(-0.1);
        assert_eq!(
            intensity.unwrap_err(),
            AUTDInternalError::DutyRatioOutOfRange(-0.1)
        );

        let intensity = EmitIntensity::new_duty_ratio(0.6);
        assert_eq!(
            intensity.unwrap_err(),
            AUTDInternalError::DutyRatioOutOfRange(0.6)
        );
    }

    #[test]
    fn test_new_pulse_width() {
        for i in 0..=256 {
            let intensity = TryIntoEmitIntensity::try_into(i);
            if i == 1 {
                assert_eq!(
                    intensity.unwrap_err(),
                    AUTDInternalError::PulseWidthOutOfRange(1)
                );
            } else {
                assert!(intensity.is_ok());
                let intensity = intensity.unwrap();
                assert_eq!(intensity.pulse_width(), i);
            }
        }

        let intensity = EmitIntensity::new_pulse_width(257);
        assert_eq!(
            intensity.unwrap_err(),
            AUTDInternalError::PulseWidthOutOfRange(257)
        );
    }

    #[test]
    fn test_normalized() {
        for i in 0..=256 {
            if i == 1 {
                continue;
            }
            let intensity = EmitIntensity::new_pulse_width(i).unwrap();
            assert_eq!(intensity.normalized(), (i as float / 512.0 * PI).sin());
        }
    }

    #[test]
    fn test_duty_ratio() {
        for i in 0..=256 {
            if i == 1 {
                continue;
            }
            let intensity = EmitIntensity::new_pulse_width(i).unwrap();
            assert_eq!(intensity.duty_ratio(), i as float / 512.0);
        }
    }

    #[test]
    fn test_pulse_width() {
        for i in 0..=256 {
            if i == 1 {
                continue;
            }
            let intensity = EmitIntensity::new_pulse_width(i).unwrap();
            assert_eq!(intensity.pulse_width(), i);
        }
    }

    #[test]
    fn test_try_into() {
        let intensity = EmitIntensity::new_pulse_width(0).unwrap();
        assert!(TryIntoEmitIntensity::try_into(intensity).is_ok());
    }

    #[test]
    fn test_clone() {
        let intensity = EmitIntensity::new_pulse_width(0).unwrap();
        assert_eq!(intensity.pulse_width(), intensity.clone().pulse_width());
    }

    #[test]
    fn test_debug() {
        let intensity = EmitIntensity::new_pulse_width(0).unwrap();
        assert_eq!(
            format!("{:?}", intensity),
            "EmitIntensity { pulse_width: 0 }"
        );
    }
}
