/*
 * File: sdp.rs
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
pub unsafe extern "C" fn AUTDGainHoloSDP(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(SDP, NalgebraBackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, SDP<NalgebraBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPWithAlpha(holo: GainPtr, alpha: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<NalgebraBackend>).with_alpha(alpha))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPWithLambda(holo: GainPtr, lambda: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<NalgebraBackend>).with_lambda(lambda))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPWithRepeat(holo: GainPtr, repeat: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<NalgebraBackend>).with_repeat(repeat as _))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{constraint::*, nalgebra_backend::*};

    #[test]
    fn test_holo_sdp() {
        unsafe {
            let backend = AUTDNalgebraBackend();

            let size = 2;
            let points = [10., 20., 30., 40., 50., 60.];
            let amps = vec![1.; size];

            let holo = AUTDGainHoloSDP(backend, points.as_ptr(), amps.as_ptr(), size as _);
            let constraint = AUTDGainHoloConstraintDotCare();
            let holo = AUTDGainHoloSDPWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintNormalize();
            let holo = AUTDGainHoloSDPWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintUniform(1.);
            let holo = AUTDGainHoloSDPWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintClamp(0., 1.);
            let holo = AUTDGainHoloSDPWithConstraint(holo, constraint);

            let holo = AUTDGainHoloSDPWithAlpha(holo, 1.);
            let holo = AUTDGainHoloSDPWithLambda(holo, 1.);
            let _ = AUTDGainHoloSDPWithRepeat(holo, 1);

            AUTDDeleteNalgebraBackend(backend);
        }
    }
}
