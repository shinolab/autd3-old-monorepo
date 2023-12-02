/*
 * File: sampling_config.rs
 * Project: src
 * Created Date: 22/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::ConstPtr;
use autd3_driver::{defined::float, error::AUTDInternalError};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SamplingConfiguration {
    pub(crate) div: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultSamplingConfig {
    pub result: SamplingConfiguration,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<autd3_driver::common::SamplingConfiguration> for SamplingConfiguration {
    fn from(value: autd3_driver::common::SamplingConfiguration) -> Self {
        Self {
            div: value.frequency_division(),
        }
    }
}

impl From<SamplingConfiguration> for autd3_driver::common::SamplingConfiguration {
    fn from(value: SamplingConfiguration) -> Self {
        Self::from_frequency_division(value.div).unwrap()
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFrequencyDivision(div: u32) -> ResultSamplingConfig {
    autd3_driver::common::SamplingConfiguration::from_frequency_division(div).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromFrequency(f: float) -> ResultSamplingConfig {
    autd3_driver::common::SamplingConfiguration::from_frequency(f).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFromPeriod(p: u64) -> ResultSamplingConfig {
    autd3_driver::common::SamplingConfiguration::from_period(std::time::Duration::from_nanos(p))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFrequencyDivision(config: SamplingConfiguration) -> u32 {
    autd3_driver::common::SamplingConfiguration::from(config).frequency_division()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFrequency(config: SamplingConfiguration) -> float {
    autd3_driver::common::SamplingConfiguration::from(config).frequency()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigPeriod(config: SamplingConfiguration) -> u64 {
    autd3_driver::common::SamplingConfiguration::from(config)
        .period()
        .as_nanos() as _
}

impl From<Result<autd3_driver::common::SamplingConfiguration, AUTDInternalError>>
    for ResultSamplingConfig
{
    fn from(r: Result<autd3_driver::common::SamplingConfiguration, AUTDInternalError>) -> Self {
        match r {
            Ok(result) => Self {
                result: result.into(),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: SamplingConfiguration { div: 0 },
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}
