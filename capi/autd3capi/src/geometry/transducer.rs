/*
 * File: transducer.rs
 * Project: devmetry
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

use autd3capi_def::{driver::geometry::Transducer, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransducer(dev: DevicePtr, idx: u32) -> TransducerPtr {
    TransducerPtr(&cast!(dev.0, Device)[idx as usize] as *const _ as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerPosition(tr: TransducerPtr, pos: *mut float) {
    let p = cast!(tr.0, Transducer).position();
    pos.add(0).write(p.x);
    pos.add(1).write(p.y);
    pos.add(2).write(p.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerRotation(tr: TransducerPtr, rot: *mut float) {
    let r = cast!(tr.0, Transducer).rotation();
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerDirectionX(tr: TransducerPtr, dir: *mut float) {
    let d = cast!(tr.0, Transducer).x_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerDirectionY(tr: TransducerPtr, dir: *mut float) {
    let d = cast!(tr.0, Transducer).y_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDTransducerDirectionZ(tr: TransducerPtr, dir: *mut float) {
    let d = cast!(tr.0, Transducer).z_direction();
    dir.add(0).write(d.x);
    dir.add(1).write(d.y);
    dir.add(2).write(d.z);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDTransducerWavelength(tr: TransducerPtr, sound_speed: float) -> float {
    cast!(tr.0, Transducer).wavelength(sound_speed)
}
