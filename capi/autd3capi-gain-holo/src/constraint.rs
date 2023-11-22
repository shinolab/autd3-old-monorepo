/*
 * File: constraint.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{holo::*, ConstraintPtr, EmitIntensity};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintDotCare() -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::DontCare)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintNormalize() -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Normalize)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintUniform(intensity: EmitIntensity) -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Uniform(intensity.into()))) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloConstraintClamp(
    min_v: EmitIntensity,
    max_v: EmitIntensity,
) -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Clamp(min_v.into(), max_v.into()))) as _)
}
