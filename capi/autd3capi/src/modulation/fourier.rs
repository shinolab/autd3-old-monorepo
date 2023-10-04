/*
 * File: fourier.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    common::{autd3::modulation::Fourier, *},
    take_mod, ModulationPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourier(m: ModulationPtr) -> ModulationPtr {
    ModulationPtr::new(Fourier::from(**take_mod!(m, Sine)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierAddComponent(
    fourier: ModulationPtr,
    m: ModulationPtr,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(fourier, Fourier).add_component(**take_mod!(m, Sine)))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{
        modulation::{sine::*, *},
        tests::*,
        *,
    };

    use autd3capi_def::{DatagramPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_fourier() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationFourier(AUTDModulationSine(150));
            let m = AUTDModulationFourierAddComponent(m, AUTDModulationSine(150));

            let m = AUTDModulationIntoDatagram(m);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDControllerSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDControllerDelete(cnt);
        }
    }
}
