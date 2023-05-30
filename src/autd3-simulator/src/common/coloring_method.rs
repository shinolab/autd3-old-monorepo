/*
 * File: coloring_method.rs
 * Project: common
 * Created Date: 22/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::color::Color;
use super::color::Hsv;

pub type ColoringMethod = fn(f32, f32, f32) -> [f32; 4];

pub fn coloring_hsv(h: f32, v: f32, a: f32) -> [f32; 4] {
    let hsv = Hsv { h, s: 1., v, a };
    hsv.rgba()
}
