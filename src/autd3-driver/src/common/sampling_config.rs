/*
 * File: sampling_config.rs
 * Project: common
 * Created Date: 14/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub use crate::defined::float;
use crate::{
    derive::prelude::AUTDInternalError,
    fpga::{FPGA_CLK_FREQ, SAMPLING_FREQ_DIV_MAX, SAMPLING_FREQ_DIV_MIN},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplingConfiguration {
    div: u32,
}

impl SamplingConfiguration {
    pub const BASE_FREQUENCY: float = FPGA_CLK_FREQ as _;

    pub const FREQ_MIN: float = Self::BASE_FREQUENCY / SAMPLING_FREQ_DIV_MAX as float;
    pub const FREQ_MAX: float = Self::BASE_FREQUENCY / SAMPLING_FREQ_DIV_MIN as float;
    pub const PERIOD_MIN: u128 =
        (1000000000. / Self::BASE_FREQUENCY * SAMPLING_FREQ_DIV_MIN as float) as u128;
    pub const PERIOD_MAX: u128 =
        (1000000000. / Self::BASE_FREQUENCY * SAMPLING_FREQ_DIV_MAX as float) as u128;

    pub fn from_frequency_division(div: u32) -> Result<Self, AUTDInternalError> {
        if !(SAMPLING_FREQ_DIV_MIN..=SAMPLING_FREQ_DIV_MAX).contains(&div) {
            Err(AUTDInternalError::SamplingFreqDivOutOfRange(
                div,
                SAMPLING_FREQ_DIV_MIN,
                SAMPLING_FREQ_DIV_MAX,
            ))
        } else {
            Ok(Self { div })
        }
    }

    pub fn from_frequency(f: float) -> Result<Self, AUTDInternalError> {
        if !(Self::FREQ_MIN..=Self::FREQ_MAX).contains(&f) {
            Err(AUTDInternalError::SamplingFreqOutOfRange(
                f,
                Self::FREQ_MIN,
                Self::FREQ_MAX,
            ))
        } else {
            let div = Self::BASE_FREQUENCY / f;
            Self::from_frequency_division(div as _)
        }
    }

    pub fn from_period(p: std::time::Duration) -> Result<Self, AUTDInternalError> {
        let p = p.as_nanos();
        if !(Self::PERIOD_MIN..=Self::PERIOD_MAX).contains(&p) {
            Err(AUTDInternalError::SamplingPeriodOutOfRange(
                p,
                Self::PERIOD_MIN,
                Self::PERIOD_MAX,
            ))
        } else {
            let div = Self::BASE_FREQUENCY / 1000000000. * p as float;
            Self::from_frequency_division(div as _)
        }
    }

    pub const fn frequency_division(&self) -> u32 {
        self.div
    }

    pub fn frequency(&self) -> float {
        Self::BASE_FREQUENCY / self.div as float
    }

    pub fn period(&self) -> std::time::Duration {
        let p = 1000000000. / Self::BASE_FREQUENCY * self.div as float;
        std::time::Duration::from_nanos(p as _)
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
    fn test_from_frequency_division() {
        let config = SamplingConfiguration::from_frequency_division(SAMPLING_FREQ_DIV_MIN);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.frequency_division(), SAMPLING_FREQ_DIV_MIN);
        assert_eq!(config.frequency(), 40e3);
        assert_eq!(config.period(), std::time::Duration::from_micros(25));

        let config = SamplingConfiguration::from_frequency_division(SAMPLING_FREQ_DIV_MIN - 1);
        assert_eq!(
            config.unwrap_err(),
            AUTDInternalError::SamplingFreqDivOutOfRange(
                SAMPLING_FREQ_DIV_MIN - 1,
                SAMPLING_FREQ_DIV_MIN,
                SAMPLING_FREQ_DIV_MAX
            )
        );
    }

    #[test]
    fn test_from_frequency() {
        let config = SamplingConfiguration::from_frequency(40e3);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.frequency_division(), 512);
        assert_eq!(config.frequency(), 40e3);
        assert_eq!(config.period(), std::time::Duration::from_micros(25));

        let config = SamplingConfiguration::from_frequency(SamplingConfiguration::FREQ_MIN - 0.1);
        assert_eq!(
            config.unwrap_err(),
            AUTDInternalError::SamplingFreqOutOfRange(
                SamplingConfiguration::FREQ_MIN - 0.1,
                SamplingConfiguration::FREQ_MIN,
                SamplingConfiguration::FREQ_MAX
            )
        );

        let config = SamplingConfiguration::from_frequency(SamplingConfiguration::FREQ_MAX + 0.1);
        assert_eq!(
            config.unwrap_err(),
            AUTDInternalError::SamplingFreqOutOfRange(
                SamplingConfiguration::FREQ_MAX + 0.1,
                SamplingConfiguration::FREQ_MIN,
                SamplingConfiguration::FREQ_MAX
            )
        );
    }

    #[test]
    fn test_from_period() {
        let config = SamplingConfiguration::from_period(std::time::Duration::from_micros(25));
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.frequency_division(), 512);
        assert_eq!(config.frequency(), 40e3);
        assert_eq!(config.period(), std::time::Duration::from_micros(25));

        let config = SamplingConfiguration::from_period(std::time::Duration::from_nanos(
            (SamplingConfiguration::PERIOD_MIN - 1) as u64,
        ));
        assert_eq!(
            config.unwrap_err(),
            AUTDInternalError::SamplingPeriodOutOfRange(
                SamplingConfiguration::PERIOD_MIN - 1,
                SamplingConfiguration::PERIOD_MIN,
                SamplingConfiguration::PERIOD_MAX
            )
        );

        let config = SamplingConfiguration::from_period(std::time::Duration::from_nanos(
            (SamplingConfiguration::PERIOD_MAX + 1) as u64,
        ));
        assert_eq!(
            config.unwrap_err(),
            AUTDInternalError::SamplingPeriodOutOfRange(
                SamplingConfiguration::PERIOD_MAX + 1,
                SamplingConfiguration::PERIOD_MIN,
                SamplingConfiguration::PERIOD_MAX
            )
        );
    }

    #[test]
    fn test_clone() {
        let config = SamplingConfiguration::from_frequency_division(512).unwrap();
        assert_eq!(config, config.clone());
    }

    #[test]
    fn test_debug() {
        let config = SamplingConfiguration::from_frequency_division(512).unwrap();
        assert_eq!(
            format!("{:?}", config),
            "SamplingConfiguration { div: 512 }"
        );
    }

    #[test]
    fn test_ord() {
        let config1 = SamplingConfiguration::from_frequency_division(512).unwrap();
        let config2 = SamplingConfiguration::from_frequency_division(513).unwrap();
        assert!(config1 < config2);
        assert_eq!(config1.min(config2), config1);
    }

    #[test]
    fn hash() {
        let config = SamplingConfiguration::from_frequency_division(512).unwrap();
        let mut s = DefaultHasher::new();
        assert_eq!(config.hash(&mut s), 512.hash(&mut s));
        s.finish();
    }
}
