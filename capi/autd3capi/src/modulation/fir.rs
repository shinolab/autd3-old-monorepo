/*
 * File: fir.rs
 * Project: modulation
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    common::{autd3::modulation::FIR, *},
    ModulationPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithLowPass(
    m: ModulationPtr,
    n_taps: u32,
    cutoff: float,
) -> ModulationPtr {
    ModulationPtr::new(Box::from_raw(m.0 as *mut Box<M>).with_low_pass(n_taps as _, cutoff))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithHighPass(
    m: ModulationPtr,
    n_taps: u32,
    cutoff: float,
) -> ModulationPtr {
    ModulationPtr::new(Box::from_raw(m.0 as *mut Box<M>).with_high_pass(n_taps as _, cutoff))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithBandPass(
    m: ModulationPtr,
    n_taps: u32,
    f_low: float,
    f_high: float,
) -> ModulationPtr {
    ModulationPtr::new(Box::from_raw(m.0 as *mut Box<M>).with_band_pass(n_taps as _, f_low, f_high))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithBandStop(
    m: ModulationPtr,
    n_taps: u32,
    f_low: float,
    f_high: float,
) -> ModulationPtr {
    ModulationPtr::new(Box::from_raw(m.0 as *mut Box<M>).with_band_stop(n_taps as _, f_low, f_high))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::{super::sine::AUTDModulationSine, *};

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_low_pass() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSine(150);
            let m = AUTDModulationWithLowPass(m, 199, 100.);
            let m = AUTDModulationIntoDatagram(m);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }

    #[test]
    fn test_high_pass() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSine(150);
            let m = AUTDModulationWithHighPass(m, 199, 100.);
            let m = AUTDModulationIntoDatagram(m);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }

    #[test]
    fn test_band_pass() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSine(150);
            let m = AUTDModulationWithBandPass(m, 199, 100., 200.);
            let m = AUTDModulationIntoDatagram(m);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }

    #[test]
    fn test_band_stop() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSine(150);
            let m = AUTDModulationWithBandStop(m, 199, 100., 200.);
            let m = AUTDModulationIntoDatagram(m);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }
}
