/*
 * File: trans_test.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{common::*, take_gain, GainPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainTransducerTest() -> GainPtr {
    GainPtr::new(TransducerTest::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainTransducerTestSet(
    trans_test: GainPtr,
    id: u32,
    phase: float,
    amp: float,
) -> GainPtr {
    GainPtr::new(take_gain!(trans_test, TransducerTest).set(id as _, phase, amp))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{DatagramHeaderPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_trans_test() {
        unsafe {
            let cnt = create_controller();

            let g = AUTDGainTransducerTest();
            let g = AUTDGainTransducerTestSet(g, 0, 1., 1.);

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
