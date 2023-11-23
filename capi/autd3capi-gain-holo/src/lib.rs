/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

pub mod constraint;
pub mod greedy;
pub mod gs;
pub mod gspat;
pub mod lm;
pub mod naive;
pub mod nalgebra_backend;
pub mod sdp;

use autd3capi_def::common::{
    driver::defined::float,
    holo::{dB, Pascal},
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSPLToPascal(value: float) -> float {
    (value * dB).as_pascal()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloPascalToSPL(value: float) -> float {
    (value * Pascal).as_spl()
}
