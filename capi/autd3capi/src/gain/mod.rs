/*
 * File: mod.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::ffi::c_char;

use autd3capi_def::{
    common::{
        autd3::core::{gain::GainFilter, Drive},
        *,
    },
    DatagramBodyPtr, GainPtr, GeometryPtr, AUTD3_ERR, AUTD3_TRUE,
};

pub mod bessel;
pub mod custom;
pub mod focus;
pub mod group;
pub mod null;
pub mod plane;
pub mod trans_test;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagram(gain: GainPtr) -> DatagramBodyPtr {
    DatagramBodyPtr::new(*Box::from_raw(gain.0 as *mut Box<G>))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCalc(
    gain: GainPtr,
    geometry: GeometryPtr,
    drives: *mut Drive,
    err: *mut c_char,
) -> i32 {
    let res = try_or_return!(
        Box::from_raw(gain.0 as *mut Box<G>).calc(cast!(geometry.0, Geo), GainFilter::All),
        err,
        AUTD3_ERR
    );
    std::ptr::copy_nonoverlapping(res.as_ptr(), drives as _, res.len());
    AUTD3_TRUE
}
