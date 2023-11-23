/*
 * File: sampling_config.rs
 * Project: src
 * Created Date: 22/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_common::{driver::defined::float, AUTDInternalError, ConstPtr};

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

impl From<autd3capi_common::driver::common::SamplingConfiguration> for SamplingConfiguration {
    fn from(value: autd3capi_common::driver::common::SamplingConfiguration) -> Self {
        Self {
            div: value.frequency_division(),
        }
    }
}

impl From<SamplingConfiguration> for autd3capi_common::driver::common::SamplingConfiguration {
    fn from(value: SamplingConfiguration) -> Self {
        Self::new_with_frequency_division(value.div).unwrap()
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigNewWithFrequencyDivision(
    div: u32,
) -> ResultSamplingConfig {
    autd3capi_common::driver::common::SamplingConfiguration::new_with_frequency_division(div).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigNewWithFrequency(f: float) -> ResultSamplingConfig {
    autd3capi_common::driver::common::SamplingConfiguration::new_with_frequency(f).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigNewWithPeriod(p: u64) -> ResultSamplingConfig {
    autd3capi_common::driver::common::SamplingConfiguration::new_with_period(
        std::time::Duration::from_nanos(p),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFrequencyDivision(config: SamplingConfiguration) -> u32 {
    autd3capi_common::driver::common::SamplingConfiguration::from(config).frequency_division()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigFrequency(config: SamplingConfiguration) -> float {
    autd3capi_common::driver::common::SamplingConfiguration::from(config).frequency()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSamplingConfigPeriod(config: SamplingConfiguration) -> u64 {
    autd3capi_common::driver::common::SamplingConfiguration::from(config)
        .period()
        .as_nanos() as _
}

impl From<Result<autd3capi_common::driver::common::SamplingConfiguration, AUTDInternalError>>
    for ResultSamplingConfig
{
    fn from(
        r: Result<autd3capi_common::driver::common::SamplingConfiguration, AUTDInternalError>,
    ) -> Self {
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
