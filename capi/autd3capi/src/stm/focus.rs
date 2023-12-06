/*
 * File: focus.rs
 * Project: stm
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
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
