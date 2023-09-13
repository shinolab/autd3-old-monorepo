/*
 * File: transducer.rs
 * Project: devmetry
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::c_char;

use autd3capi_def::{common::*, DevicePtr};

#[no_mangle]
pub unsafe extern "C" fn AUTDTransPosition(dev: DevicePtr, tr_idx: u32, pos: *mut float) {
    let p = cast!(dev.0, Dev)[tr_idx as usize].position();
    pos.add(0).write(p.x);
    pos.add(1).write(p.y);
    pos.add(2).write(p.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransRotation(dev: DevicePtr, tr_idx: u32, rot: *mut float) {
    let r = cast!(dev.0, Dev)[tr_idx as usize].rotation();
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransXDirection(dev: DevicePtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(dev.0, Dev)[tr_idx as usize].x_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransYDirection(dev: DevicePtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(dev.0, Dev)[tr_idx as usize].y_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransZDirection(dev: DevicePtr, tr_idx: u32, dir: *mut float) {
    let d = cast!(dev.0, Dev)[tr_idx as usize].z_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransFrequency(dev: DevicePtr, idx: u32) -> float {
    cast!(dev.0, Dev)[idx as usize].frequency()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransFrequency(
    dev: DevicePtr,
    idx: u32,
    value: float,
    err: *mut c_char,
) -> bool {
    try_or_return!(
        cast_mut!(dev.0, Dev)[idx as usize].set_frequency(value),
        err,
        false
    );
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransCycle(dev: DevicePtr, idx: u32) -> u16 {
    cast!(dev.0, Dev)[idx as usize].cycle()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransCycle(
    dev: DevicePtr,
    idx: u32,
    value: u16,
    err: *mut c_char,
) -> bool {
    try_or_return!(
        cast_mut!(dev.0, Dev)[idx as usize].set_cycle(value),
        err,
        false
    );
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetWavelength(dev: DevicePtr, idx: u32, sound_speed: float) -> float {
    let devmetry = cast!(dev.0, Dev);
    devmetry[idx as usize].wavelength(sound_speed)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransModDelay(dev: DevicePtr, tr_idx: u32) -> u16 {
    cast!(dev.0, Dev)[tr_idx as usize].mod_delay()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransModDelay(dev: DevicePtr, tr_idx: u32, delay: u16) {
    cast_mut!(dev.0, Dev)[tr_idx as usize].set_mod_delay(delay)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransAmpFilter(dev: DevicePtr, tr_idx: u32) -> float {
    cast!(dev.0, Dev)[tr_idx as usize].amp_filter()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransAmpFilter(dev: DevicePtr, tr_idx: u32, value: float) {
    cast_mut!(dev.0, Dev)[tr_idx as usize].set_amp_filter(value)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransPhaseFilter(dev: DevicePtr, tr_idx: u32) -> float {
    cast!(dev.0, Dev)[tr_idx as usize].phase_filter()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransPhaseFilter(dev: DevicePtr, tr_idx: u32, value: float) {
    cast_mut!(dev.0, Dev)[tr_idx as usize].set_phase_filter(value)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::{
        geometry::{device::*, *},
        tests::*,
        *,
    };

    #[test]
    fn test_transducer() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);

            let dev = AUTDGetDevice(geo, 0);

            let mut err = vec![c_char::default(); 256];

            let f = 70e3;
            assert!(AUTDSetTransFrequency(dev, 0, f, err.as_mut_ptr()));
            assert_eq!(AUTDGetTransFrequency(dev, 0), 69987.18496369073);

            let f = 4096;
            assert!(AUTDSetTransCycle(dev, 0, f, err.as_mut_ptr()));
            assert_eq!(AUTDGetTransCycle(dev, 0), f);

            assert_approx_eq::assert_approx_eq!(AUTDGetWavelength(dev, 0, 340.), 340.0 / 40e3);

            let mut v = [0., 0., 0.];
            AUTDTransPosition(dev, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);

            let mut v = [0., 0., 0., 0.];
            AUTDTransRotation(dev, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 1.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            assert_approx_eq::assert_approx_eq!(v[3], 0.);

            AUTDTransXDirection(dev, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 1.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDTransYDirection(dev, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 1.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDTransZDirection(dev, 0, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 1.);

            let delay = 0xFFFF;
            AUTDSetTransModDelay(dev, 0, delay);
            assert_eq!(delay, AUTDGetTransModDelay(dev, 0));

            AUTDFreeController(cnt);
        }
    }
}
