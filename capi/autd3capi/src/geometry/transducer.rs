/*
 * File: transducer.rs
 * Project: devmetry
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::c_char;

use autd3capi_def::{common::*, DevicePtr, TransducerPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransducer(dev: DevicePtr, tr_idx: u32) -> TransducerPtr {
    TransducerPtr(&cast!(dev.0, Dev)[tr_idx as usize] as *const _ as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransPosition(tr: TransducerPtr, pos: *mut float) {
    let p = cast!(tr.0, Tr).position();
    pos.add(0).write(p.x);
    pos.add(1).write(p.y);
    pos.add(2).write(p.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransRotation(tr: TransducerPtr, rot: *mut float) {
    let r = cast!(tr.0, Tr).rotation();
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransXDirection(tr: TransducerPtr, dir: *mut float) {
    let d = cast!(tr.0, Tr).x_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransYDirection(tr: TransducerPtr, dir: *mut float) {
    let d = cast!(tr.0, Tr).y_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransZDirection(tr: TransducerPtr, dir: *mut float) {
    let d = cast!(tr.0, Tr).z_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransFrequency(tr: TransducerPtr) -> float {
    cast!(tr.0, Tr).frequency()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransFrequency(
    tr: TransducerPtr,
    value: float,
    err: *mut c_char,
) -> bool {
    try_or_return!(cast_mut!(tr.0, Tr).set_frequency(value), err, false);
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransCycle(tr: TransducerPtr) -> u16 {
    cast!(tr.0, Tr).cycle()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSetTransCycle(
    tr: TransducerPtr,
    value: u16,
    err: *mut c_char,
) -> bool {
    try_or_return!(cast_mut!(tr.0, Tr).set_cycle(value), err, false);
    true
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetWavelength(tr: TransducerPtr, sound_speed: float) -> float {
    cast!(tr.0, Tr).wavelength(sound_speed)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransModDelay(tr: TransducerPtr) -> u16 {
    cast!(tr.0, Tr).mod_delay()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransModDelay(tr: TransducerPtr, delay: u16) {
    cast_mut!(tr.0, Tr).set_mod_delay(delay)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransAmpFilter(tr: TransducerPtr) -> float {
    cast!(tr.0, Tr).amp_filter()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransAmpFilter(tr: TransducerPtr, value: float) {
    cast_mut!(tr.0, Tr).set_amp_filter(value)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetTransPhaseFilter(tr: TransducerPtr) -> float {
    cast!(tr.0, Tr).phase_filter()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDSetTransPhaseFilter(tr: TransducerPtr, value: float) {
    cast_mut!(tr.0, Tr).set_phase_filter(value)
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

            let tr = AUTDGetTransducer(dev, 0);

            let mut err = vec![c_char::default(); 256];

            let f = 70e3;
            assert!(AUTDSetTransFrequency(tr, f, err.as_mut_ptr()));
            assert_eq!(AUTDGetTransFrequency(tr), 69987.18496369073);

            let f = 4096;
            assert!(AUTDSetTransCycle(tr, f, err.as_mut_ptr()));
            assert_eq!(AUTDGetTransCycle(tr), f);

            assert_approx_eq::assert_approx_eq!(AUTDGetWavelength(tr, 340.), 340.0 / 40e3);

            let mut v = [0., 0., 0.];
            AUTDTransPosition(tr, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);

            let mut v = [0., 0., 0., 0.];
            AUTDTransRotation(tr, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 1.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            assert_approx_eq::assert_approx_eq!(v[3], 0.);

            AUTDTransXDirection(tr, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 1.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDTransYDirection(tr, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 1.);
            assert_approx_eq::assert_approx_eq!(v[2], 0.);
            AUTDTransZDirection(tr, v.as_mut_ptr());
            assert_approx_eq::assert_approx_eq!(v[0], 0.);
            assert_approx_eq::assert_approx_eq!(v[1], 0.);
            assert_approx_eq::assert_approx_eq!(v[2], 1.);

            let delay = 0xFFFF;
            AUTDSetTransModDelay(tr, delay);
            assert_eq!(delay, AUTDGetTransModDelay(tr));

            AUTDFreeController(cnt);
        }
    }
}
