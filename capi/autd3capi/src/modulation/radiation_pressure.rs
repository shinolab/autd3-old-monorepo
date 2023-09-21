/*
 * File: radiation_pressure.rs
 * Project: modulation
 * Created Date: 21/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{common::*, ModulationPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithRadiationPressure(m: ModulationPtr) -> ModulationPtr {
    ModulationPtr::new(Box::from_raw(m.0 as *mut Box<M>).with_radiation_pressure())
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::{super::sine::AUTDModulationSine, *};

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_radiation_pressure() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSine(150);
            let m = AUTDModulationWithRadiationPressure(m);
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
