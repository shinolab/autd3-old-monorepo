/*
 * File: mod.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::ffi::c_char;

use autd3capi_def::{
    common::{autd3::driver::defined::*, *},
    *,
};

pub mod custom;
pub mod fir;
pub mod fourier;
pub mod sine;
pub mod sine_legacy;
pub mod square;
pub mod r#static;

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
pub unsafe extern "C" fn AUTDModulationSize(m: ModulationPtr, err: *mut c_char) -> i32 {
    try_or_return!(Box::from_raw(m.0 as *mut Box<M>).len(), err, AUTD3_ERR) as i32
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCalc(
    m: ModulationPtr,
    buffer: *mut float,
    err: *mut c_char,
) -> i32 {
    let res = try_or_return!(Box::from_raw(m.0 as *mut Box<M>).calc(), err, AUTD3_ERR);
    std::ptr::copy_nonoverlapping(res.as_ptr(), buffer, res.len());
    AUTD3_TRUE
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::modulation::{r#static::*, sine::*};

    #[test]
    fn test_modulation_sampling_frequency_div() {
        unsafe {
            let div = 5120;
            let m = AUTDModulationSine(150);
            let m = AUTDModulationSineWithSamplingFrequencyDivision(m, div);
            assert_eq!(div, AUTDModulationSamplingFrequencyDivision(m));
        }
    }

    #[test]
    fn test_modulation_calc() {
        unsafe {
            let m = AUTDModulationStatic();

            let mut err = vec![c_char::default(); 256];
            let size = AUTDModulationSize(m, err.as_mut_ptr());
            assert_eq!(size, 2);

            let m = AUTDModulationStatic();
            let m = AUTDModulationStaticWithAmp(m, 0.9);
            let mut buffer = vec![0.; size as _];
            assert_eq!(
                AUTDModulationCalc(m, buffer.as_mut_ptr(), err.as_mut_ptr()),
                AUTD3_TRUE
            );

            buffer.iter().for_each(|&b| {
                assert_eq!(b, 0.9);
            });
        }
    }
}
