/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_common::*;

use autd3_geometry_viewer::GeometryViewer;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryViewer() -> ConstPtr {
    Box::into_raw(Box::new(GeometryViewer::new())) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryViewerSize(
    viewer: ConstPtr,
    width: u32,
    height: u32,
) -> ConstPtr {
    let viewer = Box::from_raw(viewer as *mut GeometryViewer).window_size(width, height);
    Box::into_raw(Box::new(viewer)) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryViewerVsync(viewer: ConstPtr, vsync: bool) -> ConstPtr {
    let viewer = Box::from_raw(viewer as *mut GeometryViewer).vsync(vsync);
    Box::into_raw(Box::new(viewer)) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryViewerRun(viewer: ConstPtr, geometry: ConstPtr) -> i32 {
    Box::from_raw(viewer as *mut GeometryViewer).run(cast_without_ownership!(
        geometry,
        Geometry<DynamicTransducer>
    ))
}
