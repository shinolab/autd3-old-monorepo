/*
 * File: bessel.rs
 * Project: gain
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::geometry::{Geometry, Transducer, UnitQuaternion, Vector3};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain)]
pub struct Bessel<T: Transducer> {
    op: T::Gain,
    amp: f64,
    pos: Vector3,
    dir: Vector3,
    theta: f64,
}

impl<T: Transducer> Bessel<T> {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `point` - Start point of the beam
    /// * `dir` - Direction of the beam
    /// * `theta` - Angle between the conical wavefront of the beam and the direction
    ///
    pub fn new(pos: Vector3, dir: Vector3, theta: f64) -> Self {
        Self::with_duty(pos, dir, theta, 1.0)
    }

    /// constructor with duty
    ///
    /// # Arguments
    ///
    /// * `pos` - position of focal point
    /// * `amp` - normalized amp (from 0 to 1)
    ///
    pub fn with_duty(pos: Vector3, dir: Vector3, theta: f64, amp: f64) -> Self {
        Self {
            op: Default::default(),
            amp,
            pos,
            dir,
            theta,
        }
    }

    fn calc(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        let dir = self.dir.normalize();
        let v = Vector3::new(dir.y, -dir.x, 0.);
        let theta_v = v.norm().asin();
        let rot = if let Some(v) = v.try_normalize(1.0e-6) {
            UnitQuaternion::from_scaled_axis(v * -theta_v)
        } else {
            UnitQuaternion::identity()
        };

        let sound_speed = geometry.sound_speed;
        geometry.transducers().for_each(|tr| {
            let r = tr.position() - self.pos;
            let r = Vector3::new(r.x, r.y, r.z);
            let r = rot * r;
            let dist = self.theta.sin() * (r.x * r.x + r.y * r.y).sqrt() - self.theta.cos() * r.z;
            let phase = tr.align_phase_at(dist, sound_speed);
            self.op.set_drive(tr.id(), self.amp, phase);
        });
        Ok(())
    }
}
