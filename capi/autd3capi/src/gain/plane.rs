/*
 * File: plane.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
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
pub unsafe extern "C" fn AUTDGainPlaneWithAmp(plane: GainPtr, amp: u16) -> GainPtr {
    GainPtr::new(take_gain!(plane, Plane).with_amp(amp).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_plane() {
        unsafe {
            let cnt = create_controller();

            let g = AUTDGainPlane(0., 0., 1.);
            let g = AUTDGainPlaneWithAmp(g, 256);
            let g = AUTDGainIntoDatagram(g);

            let r = AUTDControllerSend(cnt, g, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
