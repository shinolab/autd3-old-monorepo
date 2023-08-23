/*
 * File: static.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{common::*, take_mod, ModulationPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic() -> ModulationPtr {
    ModulationPtr::new(Static::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticWithAmp(
    m: ModulationPtr,
    amp: float,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Static).with_amp(amp))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticWithSamplingFrequencyDivision(
    m: ModulationPtr,
    div: u32,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Static).with_sampling_frequency_division(div))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramBodyPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_static() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationStatic();
            let m = AUTDModulationStaticWithAmp(m, 1.);
            let div = 10240;
            let m = AUTDModulationStaticWithSamplingFrequencyDivision(m, div);

            let m = AUTDModulationIntoDatagram(m);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    DatagramBodyPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }
}
