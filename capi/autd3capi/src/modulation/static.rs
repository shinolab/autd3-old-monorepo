/*
 * File: static.rs
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
    common::{autd3::modulation::Static, *},
    take_mod, EmitIntensity, ModulationPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStatic() -> ModulationPtr {
    ModulationPtr::new(Static::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationStaticWithAmp(
    m: ModulationPtr,
    intensity: EmitIntensity,
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
            let m = AUTDModulationStaticWithAmp(m, AUTDEmitIntensityNew(255));

            let m = AUTDModulationIntoDatagram(m);

            let r = AUTDControllerSend(cnt, m, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
