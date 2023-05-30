/*
 * File: transform.rs
 * Project: common
 * Created Date: 30/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{Quaternion, Vector3};

pub fn to_gl_pos(v: Vector3) -> Vector3 {
    if cfg!(feature = "left_handed") {
        Vector3::new(v.x, v.y, -v.z)
    } else {
        v
    }
}

pub fn to_gl_rot(v: Quaternion) -> Quaternion {
    if cfg!(feature = "left_handed") {
        Quaternion::new(v.s, -v.v.x, -v.v.y, v.v.z)
    } else {
        v
    }
}
