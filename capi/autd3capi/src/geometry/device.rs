/*
 * File: device.rs
 * Project: geometry
 * Created Date: 06/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::geometry::{Quaternion, UnitQuaternion, Vector3},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDevice(geo: GeometryPtr, dev_idx: u32) -> DevicePtr {
    DevicePtr(&cast!(geo.0, Geometry)[dev_idx as usize] as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceNumTransducers(dev: DevicePtr) -> u32 {
    cast!(dev.0, Device).num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceGetSoundSpeed(dev: DevicePtr) -> float {
    cast!(dev.0, Device).sound_speed
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetSoundSpeed(dev: DevicePtr, value: float) {
    cast_mut!(dev.0, Device).sound_speed = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetSoundSpeedFromTemp(
    dev: DevicePtr,
    temp: float,
    k: float,
    r: float,
    m: float,
) {
    cast_mut!(dev.0, Device).set_sound_speed_from_temp_with(temp, k, r, m);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceGetAttenuation(dev: DevicePtr) -> float {
    cast!(dev.0, Device).attenuation
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceSetAttenuation(dev: DevicePtr, value: float) {
    cast_mut!(dev.0, Device).attenuation = value;
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceCenter(dev: DevicePtr, center: *mut float) {
    let c = cast!(dev.0, Device).center();
    center.add(0).write(c.x);
    center.add(1).write(c.y);
    center.add(2).write(c.z);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceTranslate(dev: DevicePtr, x: float, y: float, z: float) {
    cast_mut!(dev.0, Device).translate(Vector3::new(x, y, z));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceRotate(dev: DevicePtr, w: float, i: float, j: float, k: float) {
    cast_mut!(dev.0, Device).rotate(UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceAffine(
    dev: DevicePtr,
    x: float,
    y: float,
    z: float,
    w: float,
    i: float,
    j: float,
    k: float,
) {
    cast_mut!(dev.0, Device).affine(
        Vector3::new(x, y, z),
        UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)),
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeviceEnableSet(dev: DevicePtr, value: bool) {
    cast_mut!(dev.0, Device).enable = value;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDeviceEnableGet(dev: DevicePtr) -> bool {
    cast_mut!(dev.0, Device).enable
}
