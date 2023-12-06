/*
 * File: sdp.rs
 * Project: src
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

use std::rc::Rc;

use autd3capi_def::{driver::geometry::Vector3, holo::*, *};

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
    constraint: EmissionConstraintPtr,
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
