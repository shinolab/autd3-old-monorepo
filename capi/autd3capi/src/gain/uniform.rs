/*
 * File: uniform.rs
 * Project: gain
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{
    common::{autd3::gain::Uniform, *},
    take_gain, GainPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniform(amp: float) -> GainPtr {
    GainPtr::new(Uniform::new(amp))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniformWithPhase(uniform: GainPtr, phase: float) -> GainPtr {
    GainPtr::new(take_gain!(uniform, Uniform).with_phase(phase))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{DatagramHeaderPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_uniform() {
        unsafe {
            let cnt = create_controller();

            let g = AUTDGainUniform(1.);
            let g = AUTDGainUniformWithPhase(g, 1.);
            let g = AUTDGainIntoDatagram(g);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }
}
