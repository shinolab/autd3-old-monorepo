/*
 * File: focus.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{common::*, take_gain, GainPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocus(x: float, y: float, z: float) -> GainPtr {
    GainPtr::new(Focus::new(Vector3::new(x, y, z)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocusWithAmp(focus: GainPtr, amp: float) -> GainPtr {
    GainPtr::new(take_gain!(focus, Focus).with_amp(amp))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{DatagramHeaderPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_focus() {
        unsafe {
            let cnt = create_controller();

            let g = AUTDGainFocus(0., 0., 0.);
            let g = AUTDGainFocusWithAmp(g, 1.);
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
        }
    }
}
