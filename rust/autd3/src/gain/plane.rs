/*
 * File: plane.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::geometry::{Geometry, Transducer, Vector3};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain)]
pub struct Plane<T: Transducer> {
    op: T::Gain,
    amp: f64,
    dir: Vector3,
}

impl<T: Transducer> Plane<T> {
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
        Self {
            op: Default::default(),
            amp,
            dir,
        }
    }

    fn calc(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        let sound_speed = geometry.sound_speed;
        geometry.transducers().for_each(|tr| {
            let dist = self.dir.dot(tr.position());
            let phase = tr.align_phase_at(dist, sound_speed);
            self.op.set_drive(tr.id(), self.amp, phase);
        });
        Ok(())
    }
}
