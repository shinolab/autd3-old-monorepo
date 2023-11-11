/*
 * File: constraint.rs
 * Project: src
 * Created Date: 28/07/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{common::EmitIntensity, defined::float};

/// Amplitude constraint
pub enum Constraint {
    /// Do nothing (this is equivalent to `Clamp(0, 1)`)
    DontCare,
    /// Normalize the value by dividing the maximum value
    Normalize,
    /// Set all amplitudes to the specified value
    Uniform(EmitIntensity),
    /// Clamp all amplitudes to the specified range
    Clamp(float, float),
}

impl Constraint {
    pub fn convert(&self, value: float, max_value: float) -> EmitIntensity {
        match self {
            Constraint::DontCare => EmitIntensity::new_normalized(value).unwrap(),
            Constraint::Normalize => EmitIntensity::new_normalized(value / max_value).unwrap(),
            Constraint::Uniform(v) => *v,
            Constraint::Clamp(min, max) => {
                EmitIntensity::new_normalized(value.clamp(*min, *max)).unwrap()
            }
        }
    }
}
