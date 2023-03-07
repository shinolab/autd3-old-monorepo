/*
 * File: plane.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3_core::{
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive,
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain)]
pub struct Plane {
    amp: f64,
    dir: Vector3,
}

impl Plane {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `dir` - direction
    ///
    pub fn new(dir: Vector3) -> Self {
        Self::with_amp(dir, 1.0)
    }

    /// constructor with amp
    ///
    /// # Arguments
    ///
    /// * `dir` - direction
    /// * `amp` - normalized amp (from 0 to 1)
    ///
    pub fn with_amp(dir: Vector3, amp: f64) -> Self {
        Self { amp, dir }
    }
}

impl<T: Transducer> Gain<T> for Plane {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>> {
        let sound_speed = geometry.sound_speed;
        Ok(geometry
            .transducers()
            .map(|tr| {
                let dist = self.dir.dot(tr.position());
                let phase = dist * tr.wavenumber(sound_speed);
                Drive {
                    phase,
                    amp: self.amp,
                }
            })
            .collect())
    }
}
