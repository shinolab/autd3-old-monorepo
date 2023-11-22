/*
 * File: focus.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{
    common::{autd3::gain::Focus, driver::geometry::Vector3, *},
    take_gain, EmitIntensity, GainPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocus(x: float, y: float, z: float) -> GainPtr {
    GainPtr::new(Focus::new(Vector3::new(x, y, z)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocusWithIntensity(
    focus: GainPtr,
    intensity: EmitIntensity,
) -> GainPtr {
    GainPtr::new(take_gain!(focus, Focus).with_intensity(intensity))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{AUTDEmitIntensityNew, DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_focus() {
        unsafe {
            let cnt = create_controller();

            let g = AUTDGainFocus(0., 0., 0.);
            let g = AUTDGainFocusWithIntensity(g, AUTDEmitIntensityNew(255));
            let g = AUTDGainIntoDatagram(g);

            let r = AUTDControllerSend(cnt, g, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
