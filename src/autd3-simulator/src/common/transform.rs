/*
 * File: transform.rs
 * Project: common
 * Created Date: 30/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use cgmath::InnerSpace;

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

pub fn quaternion_to(v: Vector3, to: Vector3) -> Quaternion {
    let a = v.normalize();
    let b = to.normalize();
    let c = b.cross(a).normalize();
    if c.x.is_nan() || c.y.is_nan() || c.z.is_nan() {
        return Quaternion::new(1., 0., 0., 0.);
    }
    let ip = a.dot(b);
    const EPS: f32 = 1e-4;
    if c.magnitude() < EPS || 1. < ip {
        if ip < EPS - 1. {
            let a2 = Vector3::new(-a.y, a.z, a.x);
            let c2 = a2.cross(a).normalize();
            return Quaternion::new(0., c2.x, c2.y, c2.z);
        }
        return Quaternion::new(1., 0., 0., 0.);
    }
    let e = c * (0.5 * (1. - ip)).sqrt();
    Quaternion::new((0.5 * (1. + ip)).sqrt(), e.x, e.y, e.z)
}
