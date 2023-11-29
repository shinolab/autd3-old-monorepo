/*
 * File: rotation.rs
 * Project: geometry
 * Created Date: 26/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::geometry::UnitQuaternion;
use autd3capi_def::driver::{
    defined::float,
    geometry::{EulerAngle, Rad},
};

#[no_mangle]
pub unsafe extern "C" fn AUTDRotationFromEulerZYZ(x: float, y: float, z: float, rot: *mut float) {
    let r = UnitQuaternion::from(EulerAngle::ZYZ(x * Rad, y * Rad, z * Rad));
    rot.add(0).write(r.w);
    rot.add(1).write(r.i);
    rot.add(2).write(r.j);
    rot.add(3).write(r.k);
}
