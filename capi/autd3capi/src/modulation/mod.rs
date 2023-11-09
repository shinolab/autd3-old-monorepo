/*
 * File: mod.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{common::*, *};

pub mod cache;
pub mod custom;
pub mod fir;
pub mod fourier;
pub mod radiation_pressure;
pub mod sine;
pub mod sine_legacy;
pub mod square;
pub mod r#static;
pub mod transform;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingFrequencyDivision(m: ModulationPtr) -> u32 {
    Box::from_raw(m.0 as *mut Box<M>).sampling_frequency_division() as _
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::modulation::sine::*;

    #[test]
    fn test_modulation_sampling_frequency_div() {
        unsafe {
            let div = 5120;
            let m = AUTDModulationSine(150);
            let m = AUTDModulationSineWithSamplingFrequencyDivision(m, div);
            assert_eq!(div, AUTDModulationSamplingFrequencyDivision(m));
        }
    }
}
