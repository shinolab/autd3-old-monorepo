/*
 * File: mod.rs
 * Project: geometry
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

pub mod transducer;

use autd3capi_def::{common::*, ControllerPtr, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetGeometry(cnt: ControllerPtr) -> GeometryPtr {
    GeometryPtr(cast!(cnt.0, Cnt).geometry() as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetSoundSpeed(geo: GeometryPtr) -> float {
    cast!(geo.0, Geo).sound_speed
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeed(geo: GeometryPtr, value: float) {
    cast_mut!(geo.0, Geo).sound_speed = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetSoundSpeedFromTemp(
    geo: GeometryPtr,
    temp: float,
    k: float,
    r: float,
    m: float,
) {
    cast_mut!(geo.0, Geo).set_sound_speed_from_temp_with(temp, k, r, m);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetAttenuation(geo: GeometryPtr) -> float {
    cast!(geo.0, Geo).attenuation
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetAttenuation(geo: GeometryPtr, value: float) {
    cast_mut!(geo.0, Geo).attenuation = value;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNumTransducers(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geo).num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNumDevices(geo: GeometryPtr) -> u32 {
    cast!(geo.0, Geo).num_devices() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenter(geo: GeometryPtr, center: *mut float) {
    let c = cast!(geo.0, Geo).center();
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGeometryCenterOf(geo: GeometryPtr, dev_idx: u32, center: *mut float) {
    let c = cast!(geo.0, Geo).center_of(dev_idx as usize);
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

            let c = 300e3;
            AUTDSetSoundSpeed(geo, c);
            assert_eq!(c, AUTDGetSoundSpeed(geo));

            AUTDSetSoundSpeedFromTemp(geo, 15.0, 1.4, 8.314_463, 28.9647e-3);
            assert_approx_eq::assert_approx_eq!(AUTDGetSoundSpeed(geo), 340295.27186788846);

            let num_transducers = AUTDNumTransducers(geo);
            assert_eq!(num_transducers, 2 * 249);
            let num_devices = AUTDNumDevices(geo) as usize;
            assert_eq!(num_devices, 2);

            let mut v = [0., 0., 0.];
            AUTDGeometryCenter(geo, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 86.62522088353415);
            assert_approx_eq::assert_approx_eq!(v[1], 66.71325301204786);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDGeometryCenterOf(geo, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 86.62522088353415);
            assert_approx_eq::assert_approx_eq!(v[1], 66.71325301204786);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);

            AUTDFreeController(cnt);
        }
    }
}
