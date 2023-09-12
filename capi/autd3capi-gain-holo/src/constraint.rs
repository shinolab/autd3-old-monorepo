/*
 * File: constraint.rs
 * Project: src
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

use autd3capi_def::{common::*, holo::*, ConstraintPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloDotCareConstraint() -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::DontCare)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNormalizeConstraint() -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Normalize)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloUniformConstraint(value: float) -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Uniform(value))) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloClampConstraint(min_v: float, max_v: float) -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Clamp(min_v, max_v))) as _)
}
