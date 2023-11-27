/*
 * File: uniform.rs
 * Project: gain
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/11/2023
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
pub unsafe extern "C" fn AUTDGainUniform(intensity: u8) -> GainPtr {
    GainPtr::new(Uniform::new(intensity))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniformWithPhase(uniform: GainPtr, phase: float) -> GainPtr {
    GainPtr::new(take_gain!(uniform, Uniform).with_phase(phase))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_uniform() {
        unsafe {
            let cnt = create_controller();

            let g = AUTDGainUniform(0xFF);
            let g = AUTDGainUniformWithPhase(g, 1.);
            let g = AUTDGainIntoDatagram(g);

            let r = AUTDControllerSend(cnt, g, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
