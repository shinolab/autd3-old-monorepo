/*
 * File: camera_helper.rs
 * Project: src
 * Created Date: 26/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use camera_controllers::Camera;
use cgmath::{Deg, Euler};

use crate::{Quaternion, Vector3};

pub fn set_camera_angle(camera: &mut Camera<f32>, angle: Vector3) {
    let rotation = Quaternion::from(Euler {
        x: Deg(angle.x),
        y: Deg(angle.y),
        z: Deg(angle.z),
    });

    camera.right = (rotation * Vector3::unit_x()).into();
    camera.up = (rotation * Vector3::unit_y()).into();
    camera.forward = (rotation * Vector3::unit_z()).into();
}
