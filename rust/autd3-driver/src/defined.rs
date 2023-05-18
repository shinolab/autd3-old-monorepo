/*
 * File: defined.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
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

pub const VERSION_NUM_MAJOR: u8 = 0x88;
pub const VERSION_NUM_MINOR: u8 = 0x01;

pub const MAX_CYCLE: u16 = 8191;

pub const SAMPLING_FREQ_DIV_MIN: u32 = 4096;

pub const MOD_BUF_SIZE_MAX: usize = 65536;

pub const FOCUS_STM_BUF_SIZE_MAX: usize = 65536;
pub const GAIN_STM_BUF_SIZE_MAX: usize = 1024;
pub const GAIN_STM_LEGACY_BUF_SIZE_MAX: usize = 2048;
