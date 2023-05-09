/*
 * File: null.rs
 * Project: gain
 * Created Date: 01/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    gain::Gain,
    geometry::{Geometry, Transducer},
    Drive,
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain, Default)]
pub struct Null {}

impl Null {
    /// constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Gain<T> for Null {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(geometry
            .transducers()
            .map(|_| Drive { phase: 0., amp: 0. })
            .collect())
    }
}
