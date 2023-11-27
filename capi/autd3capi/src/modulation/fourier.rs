/*
 * File: fourier.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    common::{
        autd3::modulation::{Fourier, Sine},
        *,
    },
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
    use super::*;

    use crate::{
        modulation::{sine::*, *},
        tests::*,
        *,
    };

    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_fourier() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationFourier(AUTDModulationSine(150));
            let m = AUTDModulationFourierAddComponent(m, AUTDModulationSine(150));

            let m = AUTDModulationIntoDatagram(m);

            let r = AUTDControllerSend(cnt, m, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
