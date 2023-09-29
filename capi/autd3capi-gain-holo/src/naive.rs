/*
 * File: naive.rs
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

use std::rc::Rc;

use autd3capi_def::{
    common::*, create_holo, holo::*, take_gain, BackendPtr, ConstraintPtr, GainPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaive(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(Naive, NalgebraBackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaiveWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, Naive<NalgebraBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{constraint::*, nalgebra_backend::*};

    #[test]
    fn test_holo_naive() {
        unsafe {
            let backend = AUTDNalgebraBackend();

            let size = 2;
            let points = [10., 20., 30., 40., 50., 60.];
            let amps = vec![1.; size];

            let holo = AUTDGainHoloNaive(backend, points.as_ptr(), amps.as_ptr(), size as _);
            let constraint = AUTDGainHoloConstraintDotCare();
            let holo = AUTDGainHoloNaiveWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintNormalize();
            let holo = AUTDGainHoloNaiveWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintUniform(1.);
            let holo = AUTDGainHoloNaiveWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintClamp(0., 1.);
            let _ = AUTDGainHoloNaiveWithConstraint(holo, constraint);

            AUTDDeleteNalgebraBackend(backend);
        }
    }
}
