/*
 * File: mod.rs
 * Project: geometry
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

pub mod device;
pub mod rotation;
pub mod transducer;

use autd3capi_def::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometry(cnt: ControllerPtr) -> GeometryPtr {
    GeometryPtr(&cast!(cnt.0, Cnt).geometry as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryNumDevices(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geometry).num_devices() as _
}
