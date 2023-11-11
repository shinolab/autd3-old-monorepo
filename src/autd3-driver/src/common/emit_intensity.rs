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
pub struct Normalized {
    value: float,
}

impl Normalized {
    pub fn value(&self) -> float {
        self.value
    }
}

#[derive(Clone, Copy)]
pub struct NormalizedCorrected {
    value: float,
    alpha: float,
}

impl NormalizedCorrected {
    pub fn value(&self) -> float {
        self.value
    }

    pub fn alpha(&self) -> float {
        self.alpha
    }
}

#[derive(Clone, Copy)]
pub struct DutyRatio {
    value: float,
}

impl DutyRatio {
    pub fn value(&self) -> float {
        self.value
    }
}

#[derive(Clone, Copy)]
pub struct PulseWidth {
    value: u16,
}

impl PulseWidth {
    pub fn value(&self) -> u16 {
        self.value
    }
}

#[derive(Clone, Copy)]
pub enum EmitIntensity {
    Normalized(Normalized),
    NormalizedCorrected(NormalizedCorrected),
    DutyRatio(DutyRatio),
    PulseWidth(PulseWidth),
}

impl EmitIntensity {
    pub const MAX: EmitIntensity = EmitIntensity::PulseWidth(PulseWidth { value: 256 });
    pub const MIN: EmitIntensity = EmitIntensity::PulseWidth(PulseWidth { value: 0 });
    pub const DEFAULT_CORRECTED_ALPHA: float = 0.803;

    pub fn normalized(&self) -> float {
        match self {
            Self::Normalized(Normalized { value }) => *value,
            Self::NormalizedCorrected(NormalizedCorrected { value, alpha }) => {
                value.powf(1. / alpha)
            }
            Self::DutyRatio(DutyRatio { value }) => (value * PI).sin(),
            Self::PulseWidth(PulseWidth { value }) => (*value as float / 512.0 * PI).sin(),
        }
    }

    pub fn duty_ratio(&self) -> float {
        match self {
            Self::Normalized(Normalized { value }) => value.asin() / PI,
            Self::NormalizedCorrected(NormalizedCorrected { value, alpha }) => {
                value.powf(1. / alpha).asin() / PI
            }
            Self::DutyRatio(DutyRatio { value }) => *value,
            Self::PulseWidth(PulseWidth { value }) => *value as float / 512.0,
        }
    }

    pub fn pulse_width(&self) -> u16 {
        match match self {
            Self::Normalized(Normalized { value }) => (value.asin() / PI * 512.0).round() as u16,
            Self::NormalizedCorrected(NormalizedCorrected { value, alpha }) => {
                (value.powf(1. / alpha).asin() / PI * 512.0).round() as u16
            }
            Self::DutyRatio(DutyRatio { value }) => (*value * 512.0).round() as u16,
            Self::PulseWidth(PulseWidth { value }) => *value,
        } {
            1 => 0,
            r => r,
        }
    }

    pub fn new_normalized(value: float) -> Result<Self, AUTDInternalError> {
        if !(0.0..=1.0).contains(&value) {
            Err(AUTDInternalError::AmplitudeOutOfRange(value))
        } else {
            Ok(EmitIntensity::Normalized(Normalized { value }))
        }
    }

    pub fn new_normalized_clamped(value: float) -> Self {
        EmitIntensity::Normalized(Normalized {
            value: value.clamp(0.0, 1.0),
        })
    }

    pub fn new_normalized_corrected(value: float) -> Result<Self, AUTDInternalError> {
        if !(0.0..=1.0).contains(&value) {
            Err(AUTDInternalError::AmplitudeOutOfRange(value))
        } else {
            Ok(Self::NormalizedCorrected(NormalizedCorrected {
                value,
                alpha: Self::DEFAULT_CORRECTED_ALPHA,
            }))
        }
    }

    pub fn new_normalized_corrected_with_alpha(
        value: float,
        alpha: float,
    ) -> Result<Self, AUTDInternalError> {
        if !(0.0..=1.0).contains(&value) {
            Err(AUTDInternalError::AmplitudeOutOfRange(value))
        } else {
            Ok(Self::NormalizedCorrected(NormalizedCorrected {
                value,
                alpha,
            }))
        }
    }

    pub fn new_duty_ratio(value: float) -> Result<Self, AUTDInternalError> {
        if !(0.0..=0.5).contains(&value) {
            Err(AUTDInternalError::DutyRatioOutOfRange(value))
        } else {
            Ok(Self::DutyRatio(DutyRatio { value }))
        }
    }

    pub fn new_pulse_width(value: u16) -> Result<Self, AUTDInternalError> {
        if value == 1 {
            Err(AUTDInternalError::PulseWidthOutOfRange(value))
        } else if !(0..=256).contains(&value) {
            Err(AUTDInternalError::PulseWidthOutOfRange(value))
        } else {
            Ok(EmitIntensity::PulseWidth(PulseWidth { value }))
        }
    }
}

pub trait TryIntoEmittIntensity {
    fn try_into(self) -> Result<EmitIntensity, AUTDInternalError>;
}

impl TryIntoEmittIntensity for float {
    fn try_into(self) -> Result<EmitIntensity, AUTDInternalError> {
        EmitIntensity::new_normalized(self)
    }
}

impl TryIntoEmittIntensity for u16 {
    fn try_into(self) -> Result<EmitIntensity, AUTDInternalError> {
        EmitIntensity::new_pulse_width(self)
    }
}

impl TryIntoEmittIntensity for EmitIntensity {
    fn try_into(self) -> Result<EmitIntensity, AUTDInternalError> {
        Ok(self)
    }
}
