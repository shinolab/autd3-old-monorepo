/*
 * File: plane.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{common::*, take_gain, GainPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlane(nx: float, ny: float, nz: float) -> GainPtr {
    GainPtr::new(Plane::new(Vector3::new(nx, ny, nz)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainPlaneWithAmp(plane: GainPtr, amp: float) -> GainPtr {
    GainPtr::new(take_gain!(plane, Plane).with_amp(amp))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_plane() {
        unsafe {
            let cnt = create_controller();

            let g = AUTDGainPlane(0., 0., 1.);
            let g = AUTDGainPlaneWithAmp(g, 1.);
            let g = AUTDGainIntoDatagram(g);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramPtr(std::ptr::null()),
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
