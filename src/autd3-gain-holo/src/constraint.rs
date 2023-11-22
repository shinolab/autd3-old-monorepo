/*
 * File: constraint.rs
 * Project: src
 * Created Date: 28/07/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{common::EmitIntensity, defined::float};

/// Amplitude constraint
pub enum Constraint {
    /// Do nothing (this is equivalent to `Clamp(EmitIntensity::MIN, EmitIntensity::MAX)`)
    DontCare,
    /// Normalize the value by dividing the maximum value
    Normalize,
    /// Set all amplitudes to the specified value
    Uniform(EmitIntensity),
    /// Clamp all amplitudes to the specified range
    Clamp(EmitIntensity, EmitIntensity),
}

impl Constraint {
    pub fn convert(&self, value: float, max_value: float) -> EmitIntensity {
        match self {
            Constraint::DontCare => {
                EmitIntensity::new((value * 255.).round().clamp(0., 255.) as u8)
            }
            Constraint::Normalize => EmitIntensity::new((value / max_value * 255.).round() as u8),
            Constraint::Uniform(v) => *v,
            Constraint::Clamp(min, max) => EmitIntensity::new(
                (value * 255.)
                    .round()
                    .clamp(min.value() as float, max.value() as float) as u8,
            ),
        }
    }
}
