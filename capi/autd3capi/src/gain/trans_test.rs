/*
 * File: trans_test.rs
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
    common::{autd3::gain::TransducerTest, *},
    take_gain, EmitIntensity, GainPtr, TransducerPtr,
};
use driver::geometry::Transducer;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainTransducerTest() -> GainPtr {
    GainPtr::new(TransducerTest::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainTransducerTestSet(
    trans_test: GainPtr,
    tr: TransducerPtr,
    phase: float,
    intensity: EmitIntensity,
) -> GainPtr {
    GainPtr::new(take_gain!(trans_test, TransducerTest).set(
        cast!(tr.0, Transducer),
        phase,
        intensity,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        gain::*,
        geometry::{device::AUTDDevice, transducer::AUTDTransducer, AUTDGeometry},
        tests::*,
        *,
    };

    use autd3capi_def::{AUTDEmitIntensityNew, DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_trans_test() {
        unsafe {
            let cnt = create_controller();

            let geo = AUTDGeometry(cnt);
            let dev = AUTDDevice(geo, 0);
            let tr = AUTDTransducer(dev, 0);

            let g = AUTDGainTransducerTest();
            let g = AUTDGainTransducerTestSet(g, tr, 1., AUTDEmitIntensityNew(255));

            let g = AUTDGainIntoDatagram(g);

            let r = AUTDControllerSend(cnt, g, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
