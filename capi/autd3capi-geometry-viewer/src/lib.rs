/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::c_char;

use autd3capi_def::{common::*, GeometryPtr};

use autd3_geometry_viewer::GeometryViewer;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryViewerPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryViewer() -> GeometryViewerPtr {
    GeometryViewerPtr(Box::into_raw(Box::new(GeometryViewer::new())) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryViewerWithSize(
    viewer: GeometryViewerPtr,
    width: u32,
    height: u32,
) -> GeometryViewerPtr {
    let viewer = Box::from_raw(viewer.0 as *mut GeometryViewer).with_window_size(width, height);
    GeometryViewerPtr(Box::into_raw(Box::new(viewer)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryViewerWithVsync(
    viewer: GeometryViewerPtr,
    vsync: bool,
) -> GeometryViewerPtr {
    let viewer = Box::from_raw(viewer.0 as *mut GeometryViewer).with_vsync(vsync);
    GeometryViewerPtr(Box::into_raw(Box::new(viewer)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryViewerRun(
    viewer: GeometryViewerPtr,
    geometry: GeometryPtr,
    err: *mut c_char,
) -> i32 {
    try_or_return!(
        Box::from_raw(viewer.0 as *mut GeometryViewer)
            .run(cast!(geometry.0, Geometry<DynamicTransducer>)),
        err,
        -1
    )
}
