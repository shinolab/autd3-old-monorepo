/*
 * File: focus.rs
 * Project: stm
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::{
        datagram::{FocusSTM, STMProps},
        geometry::Vector3,
    },
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocus(
    props: STMPropsPtr,
    points: *const float,
    intensities: *const u8,
    size: u64,
) -> ResultDatagram {
    FocusSTM::from_props(*Box::from_raw(props.0 as *mut STMProps))
        .add_foci_from_iter((0..size as usize).map(|i| {
            let p = Vector3::new(
                points.add(i * 3).read(),
                points.add(i * 3 + 1).read(),
                points.add(i * 3 + 2).read(),
            );
            let intensities = *intensities.add(i);
            (p, intensities)
        }))
        .into()
}

#[cfg(test)]
mod tests {

    use autd3capi_def::AUTD3_TRUE;

    use super::*;

    use crate::{stm::*, tests::*, *};

    #[test]
    fn test_focus_stm() {
        unsafe {
            let cnt = create_controller();

            let props = AUTDSTMPropsNew(1.);

            let len = 2;
            let points = vec![0.; len * 3];
            let intensities = vec![0xFF; len];

            let stm = AUTDSTMFocus(props, points.as_ptr(), intensities.as_ptr(), len as _).result;

            let r = AUTDControllerSend(cnt, stm, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
