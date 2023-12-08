/*
 * File: mod.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::*;

pub mod cache;
pub mod custom;
pub mod fourier;
pub mod radiation_pressure;
pub mod sine;
pub mod square;
pub mod r#static;
pub mod transform;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingConfig(m: ModulationPtr) -> SamplingConfiguration {
    Box::from_raw(m.0 as *mut Box<M>).sampling_config().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagram(m: ModulationPtr) -> DatagramPtr {
    DatagramPtr::new(*Box::from_raw(m.0 as *mut Box<M>))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSize(m: ModulationPtr) -> ResultI32 {
    Box::from_raw(m.0 as *mut Box<M>).len().into()
}

#[repr(u8)]
pub enum SamplingMode {
    ExactFrequency = 0,
    SizeOptimized = 1,
}

impl From<SamplingMode> for autd3::modulation::SamplingMode {
    fn from(mode: SamplingMode) -> Self {
        match mode {
            SamplingMode::ExactFrequency => autd3::modulation::SamplingMode::ExactFrequency,
            SamplingMode::SizeOptimized => autd3::modulation::SamplingMode::SizeOptimized,
        }
    }
}
