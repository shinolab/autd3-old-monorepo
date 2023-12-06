/*
 * File: mod.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3capi_def::{driver::datagram::GainFilter, *};

pub mod bessel;
pub mod custom;
pub mod focus;
pub mod group;
pub mod null;
pub mod plane;
pub mod trans_test;
pub mod uniform;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagram(gain: GainPtr) -> DatagramPtr {
    DatagramPtr::new(*Box::from_raw(gain.0 as *mut Box<G>))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCalc(
    gain: GainPtr,
    geometry: GeometryPtr,
) -> ResultGainCalcDrivesMap {
    let geo = cast!(geometry.0, Geometry);
    Box::from_raw(gain.0 as *mut Box<G>)
        .calc(geo, GainFilter::All)
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcGetResult(
    src: GainCalcDrivesMapPtr,
    dst: *mut Drive,
    idx: u32,
) {
    let idx = idx as usize;
    let src = cast!(src.0, HashMap<usize, Vec<Drive>>);
    std::ptr::copy_nonoverlapping(src[&idx].as_ptr(), dst, src[&idx].len());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcFreeResult(src: GainCalcDrivesMapPtr) {
    let _ = Box::from_raw(src.0 as *mut HashMap<usize, Vec<Drive>>);
}
