/*
 * File: lib.rs
 * Project: src
 * Created Date: 17/12/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 *
 */

mod camera_helper;
mod device_viewer;
mod imgui_renderer;
mod model;
mod renderer;
mod settings;
mod viewer;

pub use settings::Settings;
pub use viewer::GeometryViewer;

pub type Vector2 = cgmath::Vector2<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Vector4 = cgmath::Vector4<f32>;
pub type Quaternion = cgmath::Quaternion<f32>;
pub type Matrix3 = cgmath::Matrix3<f32>;
pub type Matrix4 = cgmath::Matrix4<f32>;

#[cfg(feature = "use_meter")]
pub(crate) const SCALE: f32 = 1e-3;
#[cfg(not(feature = "use_meter"))]
pub(crate) const SCALE: f32 = 1.;

pub(crate) const GL_SCALE: f32 = 1e-3 / SCALE;

#[cfg(feature = "left_handed")]
pub(crate) const ZPARITY: f32 = -1.;
#[cfg(not(feature = "left_handed"))]
pub(crate) const ZPARITY: f32 = 1.;
