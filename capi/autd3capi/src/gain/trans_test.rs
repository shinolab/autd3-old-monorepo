/*
 * File: trans_test.rs
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
pub unsafe extern "C" fn AUTDGainTransducerTest() -> GainPtr {
    GainPtr::new(TransducerTest::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainTransducerTestSet(
    trans_test: GainPtr,
    dev_idx: u32,
    tr_idx: u32,
    phase: float,
    pulse_width: u16,
) -> GainPtr {
    GainPtr::new(
        take_gain!(trans_test, TransducerTest)
            .set(dev_idx as _, tr_idx as _, phase, pulse_width)
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_trans_test() {
        unsafe {
            let cnt = create_controller();

            let g = AUTDGainTransducerTest();
            let g = AUTDGainTransducerTestSet(g, 0, 0, 1., 256);

            let g = AUTDGainIntoDatagram(g);

            let r = AUTDControllerSend(cnt, g, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
