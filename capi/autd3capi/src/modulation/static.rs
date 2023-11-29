/*
 * File: static.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::Static, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic() -> ModulationPtr {
    ModulationPtr::new(Static::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticWithIntensity(
    m: ModulationPtr,
    intensity: u8,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Static).with_intensity(intensity))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_static() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationStatic();
            let m = AUTDModulationStaticWithIntensity(m, 0xFF);

            let m = AUTDModulationIntoDatagram(m);

            let r = AUTDControllerSend(cnt, m, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
