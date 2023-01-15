/*
 * File: mod.rs
 * Project: modulation
 * Created Date: 31/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

pub mod sine;
pub mod sine_legacy;
pub mod sine_pressure;
pub mod r#static;

pub use r#static::Static;
pub use sine::Sine;
pub use sine_legacy::SineLegacy;
pub use sine_pressure::SinePressure;

pub fn to_duty(amp: f64) -> u8 {
    (amp.asin() * 2.0 / std::f64::consts::PI * 255.0) as u8
}
