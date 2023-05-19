/*
 * File: constraint.rs
 * Project: src
 * Created Date: 28/07/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::float;

pub enum Constraint {
    DontCare,
    Normalize,
    Uniform(float),
    Clamp(float, float),
}

impl Constraint {
    pub fn convert(&self, value: float, max_value: float) -> float {
        match self {
            Constraint::DontCare => value,
            Constraint::Normalize => value / max_value,
            Constraint::Uniform(v) => *v,
            Constraint::Clamp(min, max) => value.clamp(*min, *max),
        }
    }
}
