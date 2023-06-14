/*
 * File: settings.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{Vector3, SCALE, ZPARITY};

#[derive(Clone)]
pub struct Settings {
    pub ambient: f32,
    pub specular: f32,
    pub light_pos_x: f32,
    pub light_pos_y: f32,
    pub light_pos_z: f32,
    pub light_power: f32,
    pub camera_pos_x: f32,
    pub camera_pos_y: f32,
    pub camera_pos_z: f32,
    pub camera_rot_x: f32,
    pub camera_rot_y: f32,
    pub camera_rot_z: f32,
    pub camera_fov: f32,
    pub camera_near_clip: f32,
    pub camera_far_clip: f32,
    pub camera_move_speed: f32,
    pub background: [f32; 4],
    pub shows: Vec<bool>,
}

impl Settings {
    pub fn new(num_dev: usize) -> Self {
        let camera_pos_x = 86.6252 * SCALE;
        let camera_pos_y = -250. * SCALE;
        let camera_pos_z = 200. * SCALE * ZPARITY;
        Self {
            ambient: 60.,
            specular: 80.,
            light_pos_x: camera_pos_x,
            light_pos_y: camera_pos_y,
            light_pos_z: camera_pos_z,
            light_power: 5.,
            camera_pos_x,
            camera_pos_y,
            camera_pos_z,
            camera_rot_x: 70. * ZPARITY,
            camera_rot_y: 0.,
            camera_rot_z: 0.,
            camera_fov: 45.,
            camera_near_clip: 0.1 * SCALE,
            camera_far_clip: 1000. * SCALE,
            camera_move_speed: 10. * SCALE,
            background: [0.3, 0.3, 0.3, 1.],
            shows: vec![true; num_dev],
        }
    }

    pub(crate) fn camera_pos(&self) -> Vector3 {
        Vector3::new(self.camera_pos_x, self.camera_pos_y, self.camera_pos_z)
    }

    pub(crate) fn camera_rot(&self) -> Vector3 {
        Vector3::new(self.camera_rot_x, self.camera_rot_y, self.camera_rot_z)
    }
}
