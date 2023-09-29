/*
 * File: greedy.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/09/2023
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

    use super::*;
    use crate::{constraint::*, nalgebra_backend::*};

    #[test]
    fn test_holo_greedy() {
        unsafe {
            let backend = AUTDNalgebraBackend();

            let size = 2;
            let points = [10., 20., 30., 40., 50., 60.];
            let amps = vec![1.; size];

            let holo = AUTDGainHoloGreedy(points.as_ptr(), amps.as_ptr(), size as _);
            let constraint = AUTDGainHoloConstraintDotCare();
            let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintNormalize();
            let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintUniform(1.);
            let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintClamp(0., 1.);
            let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);

            let _ = AUTDGainHoloGreedyWithPhaseDiv(holo, 5);

            AUTDDeleteNalgebraBackend(backend);
        }
    }
}
