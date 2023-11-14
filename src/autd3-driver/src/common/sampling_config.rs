/*
 * File: sampling_config.rs
 * Project: common
 * Created Date: 14/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
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

#[derive(Clone, Copy)]
pub struct SamplingConfiguration {
    div: u32,
}

impl SamplingConfiguration {
    pub const BASE_FREQUENCY: float = FPGA_CLK_FREQ as _;
    pub const MIN: Self = Self {
        div: SAMPLING_FREQ_DIV_MIN,
    };
    pub const MAX: Self = Self {
        div: SAMPLING_FREQ_DIV_MAX,
    };
    const FREQ_MIN: float = Self::BASE_FREQUENCY / SAMPLING_FREQ_DIV_MAX as float;
    const FREQ_MAX: float = Self::BASE_FREQUENCY / SAMPLING_FREQ_DIV_MIN as float;
    const PERIOD_MIN: u128 =
        (1000000000. / Self::BASE_FREQUENCY * SAMPLING_FREQ_DIV_MIN as float) as u128;
    const PERIOD_MAX: u128 =
        (1000000000. / Self::BASE_FREQUENCY * SAMPLING_FREQ_DIV_MAX as float) as u128;

    pub fn new_with_frequency_division(div: u32) -> Result<Self, AUTDInternalError> {
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

    pub fn new_with_frequency(f: float) -> Result<Self, AUTDInternalError> {
        if !(Self::FREQ_MIN..=Self::FREQ_MAX).contains(&f) {
            Err(AUTDInternalError::SamplingFreqOutOfRange(
                f,
                Self::FREQ_MIN,
                Self::FREQ_MAX,
            ))
        } else {
            let div = Self::BASE_FREQUENCY / f;
            Self::new_with_frequency_division(div as _)
        }
    }

    pub fn new_with_period(p: std::time::Duration) -> Result<Self, AUTDInternalError> {
        let p = p.as_nanos();
        if !(Self::PERIOD_MIN..=Self::PERIOD_MAX).contains(&p) {
            Err(AUTDInternalError::SamplingPeriodOutOfRange(
                p,
                Self::PERIOD_MIN,
                Self::PERIOD_MAX,
            ))
        } else {
            let div = Self::BASE_FREQUENCY / 1000000000. * p as float;
            Self::new_with_frequency_division(div as _)
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
