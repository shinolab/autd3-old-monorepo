/*
 * File: plane.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive,
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain, Clone, Copy)]
pub struct Plane {
    amp: float,
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
    pub fn with_amp(dir: Vector3, amp: float) -> Self {
        Self { amp, dir }
    }
}

impl<T: Transducer> Gain<T> for Plane {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let sound_speed = geometry.sound_speed;
        Ok(Self::transform(geometry, |tr| {
            let dist = self.dir.dot(tr.position());
            let phase = dist * tr.wavenumber(sound_speed);
            Drive {
                phase,
                amp: self.amp,
            }
        }))
    }
}
