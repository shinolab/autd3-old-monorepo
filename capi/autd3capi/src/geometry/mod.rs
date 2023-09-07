/*
 * File: mod.rs
 * Project: geometry
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

pub mod device;
pub mod transducer;

use autd3capi_def::{common::*, ControllerPtr, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetGeometry(cnt: ControllerPtr) -> GeometryPtr {
    GeometryPtr(cast!(cnt.0, Cnt).geometry() as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryNumTransducers(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geo).num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGeometryNumDevices(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geo).num_devices() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometrySetSoundSpeedFromTemp(
    geo: GeometryPtr,
    temp: float,
    k: float,
    r: float,
    m: float,
) {
    cast_mut!(geo.0, Geo).set_sound_speed_from_temp_with(temp, k, r, m);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenter(geo: GeometryPtr, center: *mut float) {
    let c = cast!(geo.0, Geo).center();
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{tests::*, *};

    #[test]
    fn test_geometry() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);

            let num_transducers = AUTDGeometryNumTransducers(geo);
            assert_eq!(num_transducers, 2 * 249);
            let num_devices = AUTDGeometryNumDevices(geo) as usize;
            assert_eq!(num_devices, 2);

            let mut v = [0., 0., 0.];
            AUTDGeometryCenter(geo, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 86.62522088353415);
            assert_approx_eq::assert_approx_eq!(v[1], 66.71325301204786);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);

            AUTDFreeController(cnt);
        }
    }
}
