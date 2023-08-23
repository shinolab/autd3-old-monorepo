/*
 * File: focus.rs
 * Project: stm
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3_core::stm::STMProps;
use autd3capi_def::{common::*, DatagramBodyPtr, STMPropsPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDFocusSTM(
    props: STMPropsPtr,
    points: *const float,
    shift: *const u8,
    size: u64,
) -> DatagramBodyPtr {
    DatagramBodyPtr::new(
        FocusSTM::with_props(*Box::from_raw(props.0 as *mut STMProps)).add_foci_from_iter(
            (0..size as usize).map(|i| {
                let p = Vector3::new(
                    points.add(i * 3).read(),
                    points.add(i * 3 + 1).read(),
                    points.add(i * 3 + 2).read(),
                );
                let shift = *shift.add(i);
                (p, shift)
            }),
        ),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::{stm::*, tests::*, *};

    #[test]
    fn test_focus_stm() {
        unsafe {
            let cnt = create_controller();

            let props = AUTDSTMProps(1.);

            let len = 2;
            let points = vec![0.; len * 3];
            let shifts = vec![0; len];

            let stm = AUTDFocusSTM(props, points.as_ptr(), shifts.as_ptr(), len as _);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    stm,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }
}
