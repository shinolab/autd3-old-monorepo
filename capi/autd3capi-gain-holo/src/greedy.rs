/*
 * File: greedy.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{common::*, create_holo, holo::*, take_gain, ConstraintPtr, GainPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedy(
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(Greedy, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(take_gain!(holo, Greedy).with_constraint(*Box::from_raw(constraint.0 as _)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyWithPhaseDiv(holo: GainPtr, div: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, Greedy).with_phase_div(div as _))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;
    use crate::{constraint::*, nalgebra_backend::*, tests::*};

    use autd3capi::{gain::*, *};
    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_holo_greedy() {
        unsafe {
            let cnt = create_controller();

            let backend = AUTDNalgebraBackend();

            let size = 2;
            let points = vec![10., 20., 30., 40., 50., 60.];
            let amps = vec![1.; size];

            let holo = AUTDGainHoloGreedy(points.as_ptr(), amps.as_ptr(), size as _);
            let constraint = AUTDGainHoloDotCareConstraint();
            let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloNormalizeConstraint();
            let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloUniformConstraint(1.);
            let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloClampConstraint(0., 1.);
            let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);

            let holo = AUTDGainHoloGreedyWithPhaseDiv(holo, 5);

            let holo = AUTDGainIntoDatagram(holo);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    autd3capi_def::TransMode::Legacy,
                    DatagramPtr(std::ptr::null()),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDDeleteNalgebraBackend(backend);
            AUTDFreeController(cnt);
        }
    }
}
