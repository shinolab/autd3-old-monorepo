/*
 * File: constraint.rs
 * Project: src
 * Created Date: 28/07/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

pub trait Constraint {
    fn convert(&self, v: f64, max: f64) -> f64;
}

pub struct DontCare {}

impl Constraint for DontCare {
    fn convert(&self, v: f64, _max: f64) -> f64 {
        v
    }
}

pub struct Normalize {}

impl Constraint for Normalize {
    fn convert(&self, v: f64, max: f64) -> f64 {
        v / max
    }
}

pub struct Uniform {
    v: f64,
}

impl Uniform {
    pub fn new(v: f64) -> Self {
        Self { v }
    }
}

impl Constraint for Uniform {
    fn convert(&self, _v: f64, _max: f64) -> f64 {
        self.v
    }
}

pub struct Clamp {}

impl Constraint for Clamp {
    fn convert(&self, v: f64, _max: f64) -> f64 {
        v.clamp(0.0, 1.0)
    }
}
