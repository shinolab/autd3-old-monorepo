/*
 * File: sine_legacy.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{common::*, take_mod, ModulationPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineLegacy(freq: float) -> ModulationPtr {
    ModulationPtr::new(SineLegacy::new(freq as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineLegacyWithAmp(
    m: ModulationPtr,
    amp: float,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, SineLegacy).with_amp(amp))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineLegacyWithOffset(
    m: ModulationPtr,
    offset: float,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, SineLegacy).with_offset(offset))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineLegacyWithSamplingFrequencyDivision(
    m: ModulationPtr,
    div: u32,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, SineLegacy).with_sampling_frequency_division(div))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_sine_legacy() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSineLegacy(150.);
            let m = AUTDModulationSineLegacyWithAmp(m, 1.);
            let m = AUTDModulationSineLegacyWithOffset(m, 0.5);
            let div = 10240;
            let m = AUTDModulationSineLegacyWithSamplingFrequencyDivision(m, div);

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
