/*
 * File: mod.rs
 * Project: stm
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
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
                let min = SamplingConfiguration::FREQ_MIN / size as float;
                let max = SamplingConfiguration::FREQ_MAX / size as float;
                SamplingConfiguration::from_frequency(f * size as float)
                    .map_err(|_| AUTDInternalError::STMFreqOutOfRange(size, *f, min, max))
            }
            Self::Period(p) => {
                let min = SamplingConfiguration::PERIOD_MIN as usize / size;
                let max = SamplingConfiguration::PERIOD_MAX as usize / size;
                SamplingConfiguration::from_period(std::time::Duration::from_nanos(
                    (p.as_nanos() as usize / size) as _,
                ))
                .map_err(|_| AUTDInternalError::STMPeriodOutOfRange(size, p.as_nanos(), min, max))
            }
            Self::SamplingConfiguration(s) => Ok(*s),
        }
    }
}

#[doc(hidden)]
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

    pub fn from_period(period: std::time::Duration) -> Self {
        Self {
            sampling: STMSamplingConfiguration::Period(period),
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn from_sampling_config(sampling: SamplingConfiguration) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency() {
        let config = STMSamplingConfiguration::Frequency(4e3);
        assert_eq!(config.frequency(1), 4e3);
        assert_eq!(config.frequency(2), 4e3);
        assert_eq!(config.period(1), std::time::Duration::from_micros(250));
        assert_eq!(config.period(2), std::time::Duration::from_micros(250));
        assert_eq!(
            config.sampling(1).unwrap(),
            SamplingConfiguration::from_frequency(4e3).unwrap()
        );
        assert_eq!(
            config.sampling(2).unwrap(),
            SamplingConfiguration::from_frequency(8e3).unwrap()
        );

        let config = STMSamplingConfiguration::Frequency(0.1);
        assert_eq!(config.frequency(65536), 0.1);
        assert_eq!(config.period(65536), std::time::Duration::from_secs(10));
        assert_eq!(
            config.sampling(65536).unwrap(),
            SamplingConfiguration::from_frequency(0.1 * 65536.0).unwrap()
        );
    }

    #[test]
    fn test_period() {
        let config = STMSamplingConfiguration::Period(std::time::Duration::from_micros(250));
        assert_eq!(config.frequency(1), 4e3);
        assert_eq!(config.frequency(2), 4e3);
        assert_eq!(config.period(1), std::time::Duration::from_micros(250));
        assert_eq!(config.period(2), std::time::Duration::from_micros(250));
        assert_eq!(
            config.sampling(1).unwrap(),
            SamplingConfiguration::from_frequency(4e3).unwrap()
        );
        assert_eq!(
            config.sampling(2).unwrap(),
            SamplingConfiguration::from_frequency(8e3).unwrap()
        );

        let config = STMSamplingConfiguration::Period(std::time::Duration::from_secs(10));
        assert_eq!(config.frequency(65536), 0.1);
        assert_eq!(config.period(65536), std::time::Duration::from_secs(10));
        assert_eq!(
            config.sampling(65536).unwrap(),
            SamplingConfiguration::from_period(std::time::Duration::from_nanos(
                10 * 1000 * 1000 * 1000 / 65536
            ))
            .unwrap()
        );
    }

    #[test]
    fn test_sampling() {
        let config = STMSamplingConfiguration::SamplingConfiguration(
            SamplingConfiguration::from_frequency(4e3).unwrap(),
        );
        assert_eq!(config.frequency(1), 4e3);
        assert_eq!(config.frequency(2), 2e3);
        assert_eq!(config.period(1), std::time::Duration::from_micros(250));
        assert_eq!(config.period(2), std::time::Duration::from_micros(500));
        assert_eq!(
            config.sampling(1).unwrap(),
            SamplingConfiguration::from_frequency(4e3).unwrap()
        );
        assert_eq!(
            config.sampling(2).unwrap(),
            SamplingConfiguration::from_frequency(4e3).unwrap()
        );
    }

    #[test]
    fn test_frequency_out_of_range() {
        let config = STMSamplingConfiguration::Frequency(40e3);
        assert_eq!(
            config.sampling(1),
            Ok(SamplingConfiguration::from_frequency(40e3).unwrap())
        );
        assert_eq!(
            config.sampling(2),
            Err(AUTDInternalError::STMFreqOutOfRange(
                2,
                40e3,
                SamplingConfiguration::FREQ_MIN / 2.,
                SamplingConfiguration::FREQ_MAX / 2.,
            ))
        );
    }

    #[test]
    fn test_period_out_of_range() {
        let config = STMSamplingConfiguration::Period(std::time::Duration::from_micros(25));
        assert_eq!(
            config.sampling(1),
            Ok(SamplingConfiguration::from_frequency(40e3).unwrap())
        );
        assert_eq!(
            config.sampling(2),
            Err(AUTDInternalError::STMPeriodOutOfRange(
                2,
                25000,
                SamplingConfiguration::PERIOD_MIN as usize / 2,
                SamplingConfiguration::PERIOD_MAX as usize / 2,
            ))
        );
    }
}
