/*
 * File: emit_intensity.rs
 * Project: common
 * Created Date: 11/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    defined::{float, PI},
    derive::prelude::AUTDInternalError,
};

#[derive(Clone, Copy)]
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
