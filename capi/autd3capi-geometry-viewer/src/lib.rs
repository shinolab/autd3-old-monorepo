#![allow(clippy::missing_safety_doc)]

use autd3capi_common::*;

use autd3_geometry_viewer::GeometryViewer;

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkGeometryViewer() -> ConstPtr {
    Box::into_raw(Box::new(GeometryViewer::new())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkGeometryViewerSize(
    viewer: ConstPtr,
    width: u32,
    height: u32,
) -> ConstPtr {
    let viewer = Box::from_raw(viewer as *mut GeometryViewer).window_size(width, height);
    Box::into_raw(Box::new(viewer)) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkGeometryViewerVsync(viewer: ConstPtr, vsync: bool) -> ConstPtr {
    let viewer = Box::from_raw(viewer as *mut GeometryViewer).vsync(vsync);
    Box::into_raw(Box::new(viewer)) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorRun(viewer: ConstPtr, cnt: ConstPtr) -> i32 {
    Box::from_raw(viewer as *mut GeometryViewer)
        .run(cast_without_ownership_mut!(cnt, Cnt).geometry())
}
