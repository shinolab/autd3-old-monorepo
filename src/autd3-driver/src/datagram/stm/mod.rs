/*
 * File: mod.rs
 * Project: stm
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod focus;
mod gain;

pub use focus::FocusSTM;
pub use gain::GainSTM;

use crate::{common::SamplingConfiguration, defined::float, derive::prelude::AUTDInternalError};

enum STMSamplingConfiguration {
    Frequency(float),
    Period(std::time::Duration),
    SamplingConfiguration(SamplingConfiguration),
}

impl STMSamplingConfiguration {
    pub fn frequency(&self, size: usize) -> float {
        match self {
            Self::Frequency(f) => *f,
            Self::Period(p) => 1000000000. / p.as_nanos() as float,
            Self::SamplingConfiguration(s) => s.frequency() / size as float,
        }
    }

    pub fn period(&self, size: usize) -> std::time::Duration {
        match self {
            Self::Frequency(f) => std::time::Duration::from_nanos((1000000000. / f) as _),
            Self::Period(p) => *p,
            Self::SamplingConfiguration(s) => s.period() * size as u32,
        }
    }

    pub fn sampling(&self, size: usize) -> Result<SamplingConfiguration, AUTDInternalError> {
        match self {
            Self::Frequency(f) => {
                let min = SamplingConfiguration::MIN.frequency() * size as float;
                let max = SamplingConfiguration::MAX.frequency() * size as float;
                SamplingConfiguration::new_with_frequency(f * size as float)
                    .map_err(|_| AUTDInternalError::STMFreqOutOfRange(size, *f, min, max))
            }
            Self::Period(p) => {
                let min = SamplingConfiguration::MIN.period().as_nanos() as usize / size;
                let max = SamplingConfiguration::MAX.period().as_nanos() as usize / size;
                SamplingConfiguration::new_with_period(std::time::Duration::from_nanos(
                    (p.as_nanos() as usize / size) as _,
                ))
                .map_err(|_| AUTDInternalError::STMPeriodOutOfRange(size, p.as_nanos(), min, max))
            }
            Self::SamplingConfiguration(s) => Ok(*s),
        }
    }
}

#[doc(hidden)]
/// This is used only for internal.
// #[derive(Clone, Copy)]
pub struct STMProps {
    sampling: STMSamplingConfiguration,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
}

impl STMProps {
    pub fn new(freq: float) -> Self {
        Self {
            sampling: STMSamplingConfiguration::Frequency(freq),
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn new_with_period(period: std::time::Duration) -> Self {
        Self {
            sampling: STMSamplingConfiguration::Period(period),
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn new_with_sampling_config(sampling: SamplingConfiguration) -> Self {
        Self {
            sampling: STMSamplingConfiguration::SamplingConfiguration(sampling),
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn with_start_idx(self, idx: Option<u16>) -> Self {
        Self {
            start_idx: idx,
            ..self
        }
    }

    pub fn with_finish_idx(self, idx: Option<u16>) -> Self {
        Self {
            finish_idx: idx,
            ..self
        }
    }

    pub fn start_idx(&self) -> Option<u16> {
        self.start_idx
    }

    pub fn finish_idx(&self) -> Option<u16> {
        self.finish_idx
    }

    pub fn freq(&self, size: usize) -> float {
        self.sampling.frequency(size)
    }

    pub fn period(&self, size: usize) -> std::time::Duration {
        self.sampling.period(size)
    }

    pub fn sampling_config(&self, size: usize) -> Result<SamplingConfiguration, AUTDInternalError> {
        self.sampling.sampling(size)
    }
}
