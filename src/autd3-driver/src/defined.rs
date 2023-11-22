/*
 * File: defined.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

#[cfg(feature = "single_float")]
mod float_def {
    pub use f32 as float;
    pub use std::f32::consts::PI;
}
#[cfg(not(feature = "single_float"))]
mod float_def {
    pub use f64 as float;
    pub use std::f64::consts::PI;
}

pub use float_def::*;

#[cfg(feature = "use_meter")]
mod unit {
    pub const METER: super::float = 1.0;
}
#[cfg(not(feature = "use_meter"))]
mod unit {
    pub const METER: super::float = 1000.0;
}
pub use unit::*;

pub const MILLIMETER: float = METER / 1000.0;

pub const ULTRASOUND_FREQUENCY: float = 40e3;

pub type Complex = nalgebra::Complex<float>;

pub const ABSOLUTE_THRESHOLD_OF_HEARING: float = 20e-6;

pub const T4010A1_AMPLITUDE: float = 23.77004454874038 * 300.0 * MILLIMETER; // [Pa/mm] (121.5 dB SPL @ 30cm)
