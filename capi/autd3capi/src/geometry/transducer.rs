/*
 * File: transducer.rs
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

use std::ffi::c_char;

use autd3capi_def::{common::*, GeometryPtr};

#[no_mangle]
pub unsafe extern "C" fn AUTDTransPosition(geo: GeometryPtr, tr_idx: u32, pos: *mut float) {
    let p = cast!(geo.0, Geo)[tr_idx as usize].position();
    pos.add(0).write(p.x);
    pos.add(1).write(p.y);
    pos.add(2).write(p.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransRotation(geo: GeometryPtr, tr_idx: u32, rot: *mut float) {
    let r = cast!(geo.0, Geo)[tr_idx as usize].rotation();
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransXDirection(geo: GeometryPtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(geo.0, Geo)[tr_idx as usize].x_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransYDirection(geo: GeometryPtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(geo.0, Geo)[tr_idx as usize].y_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransZDirection(geo: GeometryPtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(geo.0, Geo)[tr_idx as usize].z_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransFrequency(geo: GeometryPtr, idx: u32) -> float {
    cast!(geo.0, Geo)[idx as usize].frequency()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransFrequency(
    geo: GeometryPtr,
    idx: u32,
    value: float,
    err: *mut c_char,
) -> bool {
    try_or_return!(
        cast_mut!(geo.0, Geo)[idx as usize].set_frequency(value),
        err,
        false
    );
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransCycle(geo: GeometryPtr, idx: u32) -> u16 {
    cast!(geo.0, Geo)[idx as usize].cycle()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransCycle(
    geo: GeometryPtr,
    idx: u32,
    value: u16,
    err: *mut c_char,
) -> bool {
    try_or_return!(
        cast_mut!(geo.0, Geo)[idx as usize].set_cycle(value),
        err,
        false
    );
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetWavelength(
    geo: GeometryPtr,
    idx: u32,
    sound_speed: float,
) -> float {
    let geometry = cast!(geo.0, Geo);
    geometry[idx as usize].wavelength(sound_speed)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransModDelay(geo: GeometryPtr, tr_idx: u32) -> u16 {
    cast!(geo.0, Geo)[tr_idx as usize].mod_delay()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransModDelay(geo: GeometryPtr, tr_idx: u32, delay: u16) {
    cast_mut!(geo.0, Geo)[tr_idx as usize].set_mod_delay(delay)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::{geometry::*, tests::*, *};

    #[test]
    fn test_transducer() {
        unsafe {
            let cnt = create_controller();

            let geo = AUTDGetGeometry(cnt);

            let mut err = vec![c_char::default(); 256];

            let f = 70e3;
            assert!(AUTDSetTransFrequency(geo, 0, f, err.as_mut_ptr()));
            assert_eq!(AUTDGetTransFrequency(geo, 0), 69987.18496369073);

            let f = 4096;
            assert!(AUTDSetTransCycle(geo, 0, f, err.as_mut_ptr()));
            assert_eq!(AUTDGetTransCycle(geo, 0), f);

            assert_approx_eq::assert_approx_eq!(AUTDGetWavelength(geo, 0, 340.), 340.0 / 40e3);

            let atten = 0.1;
            AUTDSetAttenuation(geo, atten);
            assert_eq!(AUTDGetAttenuation(geo), 0.1);

            let mut v = [0., 0., 0.];
            AUTDTransPosition(geo, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDTransXDirection(geo, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 1.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDTransYDirection(geo, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 1.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDTransZDirection(geo, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 1.);

            let delay = 0xFFFF;
            AUTDSetTransModDelay(geo, 0, delay);
            assert_eq!(delay, AUTDGetTransModDelay(geo, 0));

            AUTDFreeController(cnt);
        }
    }
}
