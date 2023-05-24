/*
 * File: viewer_settings.rs
 * Project: src
 * Created Date: 26/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use crate::{Quaternion, Vector3, Vector4, SCALE, ZPARITY};
use cgmath::{Deg, Euler};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ColorMapType {
    Viridis,
    Magma,
    Inferno,
    Plasma,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewerSettings {
    pub window_width: i32,
    pub window_height: i32,
    pub vsync: bool,
    pub gpu_idx: i32,
    pub slice_pos_x: f32,
    pub slice_pos_y: f32,
    pub slice_pos_z: f32,
    pub slice_width: f32,
    pub slice_height: f32,
    pub slice_pixel_size: f32,
    pub camera_pos_x: f32,
    pub camera_pos_y: f32,
    pub camera_pos_z: f32,
    pub camera_near_clip: f32,
    pub camera_far_clip: f32,
    pub camera_move_speed: f32,
    pub sound_speed: f32,
    pub slice_rot_x: f32,
    pub slice_rot_y: f32,
    pub slice_rot_z: f32,
    pub slice_color_scale: f32,
    pub slice_alpha: f32,
    pub color_map_type: ColorMapType,
    pub show_radiation_pressure: bool,
    pub camera_rot_x: f32,
    pub camera_rot_y: f32,
    pub camera_rot_z: f32,
    pub camera_fov: f32,
    pub font_size: f32,
    pub background: [f32; 4],
    pub show_mod_plot: bool,
    pub show_mod_plot_raw: bool,
    pub mod_enable: bool,
    pub mod_auto_play: bool,
    pub stm_auto_play: bool,
    pub image_save_path: String,
    pub max_dev_num: usize,
    pub max_trans_num: usize,
    pub port: u16,
}

impl ViewerSettings {
    pub fn new() -> ViewerSettings {
        Self::default()
    }

    pub(crate) fn slice_pos(&self) -> Vector4 {
        Vector4::new(self.slice_pos_x, self.slice_pos_y, self.slice_pos_z, 1.)
    }

    pub(crate) fn slice_rotation(&self) -> Quaternion {
        Quaternion::from(Euler {
            x: Deg(self.slice_rot_x),
            y: Deg(self.slice_rot_y),
            z: Deg(self.slice_rot_z),
        })
    }

    pub(crate) fn set_camera_pos(&mut self, v: Vector3) {
        self.camera_pos_x = v.x;
        self.camera_pos_y = v.y;
        self.camera_pos_z = v.z;
    }

    pub(crate) fn set_camera_rot(&mut self, rot: Quaternion) {
        let euler = Euler::from(rot);
        self.camera_rot_x = Deg::from(euler.x).0;
        self.camera_rot_y = Deg::from(euler.y).0;
        self.camera_rot_z = Deg::from(euler.z).0;
    }
}

impl Default for ViewerSettings {
    fn default() -> Self {
        ViewerSettings {
            window_width: 1200,
            window_height: 900,
            vsync: true,
            gpu_idx: 0,
            slice_pos_x: 86.6252 * SCALE,
            slice_pos_y: 66.7133 * SCALE,
            slice_pos_z: 150.0 * SCALE * ZPARITY,
            slice_width: 300.0 * SCALE,
            slice_height: 300.0 * SCALE,
            slice_pixel_size: 1.0 * SCALE,
            camera_pos_x: 86.6252 * SCALE,
            camera_pos_y: -533.2867 * SCALE,
            camera_pos_z: 150.0 * SCALE * ZPARITY,
            camera_near_clip: 0.1 * SCALE,
            camera_far_clip: 1000. * SCALE,
            camera_move_speed: 10.0 * SCALE,
            sound_speed: 340.0e3 * SCALE,
            slice_rot_x: 90.0 * ZPARITY,
            slice_rot_y: 0.,
            slice_rot_z: 0.,
            slice_color_scale: 2.,
            slice_alpha: 1.,
            color_map_type: ColorMapType::Inferno,
            show_radiation_pressure: false,
            camera_rot_x: 90.0 * ZPARITY,
            camera_rot_y: 0.,
            camera_rot_z: 0.,
            camera_fov: 45.,
            font_size: 16.,
            background: [0.3, 0.3, 0.3, 1.],
            show_mod_plot: false,
            show_mod_plot_raw: false,
            mod_enable: false,
            mod_auto_play: false,
            stm_auto_play: false,
            image_save_path: "image.png".to_string(),
            max_dev_num: 50,
            max_trans_num: 10000,
            port: 0,
        }
    }
}
