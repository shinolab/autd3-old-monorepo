/*
 * File: constraint.rs
 * Project: src
 * Created Date: 28/07/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{common::Amplitude, defined::float};

/// Amplitude constraint
pub enum Constraint {
    /// Do nothing (this is equivalent to `Clamp(0, 1)`)
    DontCare,
    /// Normalize the value by dividing the maximum value
    Normalize,
    /// Set all amplitudes to the specified value
    Uniform(Amplitude),
    /// Clamp all amplitudes to the specified range
    Clamp(float, float),
}

impl Constraint {
    pub fn convert(&self, value: float, max_value: float) -> Amplitude {
        match self {
            Constraint::DontCare => Amplitude::new_clamped(value),
            Constraint::Normalize => Amplitude::new_clamped(value / max_value),
            Constraint::Uniform(v) => *v,
            Constraint::Clamp(min, max) => Amplitude::new_clamped(value.clamp(*min, *max)),
        }
    }
}
