/*
 * File: bessel.rs
 * Project: gain
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer, UnitQuaternion, Vector3},
    Drive,
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain, Clone, Copy)]
pub struct Bessel {
    amp: float,
    pos: Vector3,
    dir: Vector3,
    theta: float,
}

impl Bessel {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `point` - Start point of the beam
    /// * `dir` - Direction of the beam
    /// * `theta` - Angle between the conical wavefront of the beam and the direction
    ///
    pub fn new(pos: Vector3, dir: Vector3, theta: float) -> Self {
        Self {
            pos,
            dir,
            theta,
            amp: 1.0,
        }
    }

    /// set amplitude
    ///
    /// # Arguments
    ///
    /// * `amp` - normalized amp (from 0 to 1)
    ///
    pub fn with_amp(self, amp: float) -> Self {
        Self { amp, ..self }
    }
}

impl<T: Transducer> Gain<T> for Bessel {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let dir = self.dir.normalize();
        let v = Vector3::new(dir.y, -dir.x, 0.);
        let theta_v = v.norm().asin();
        let rot = if let Some(v) = v.try_normalize(1.0e-6) {
            UnitQuaternion::from_scaled_axis(v * -theta_v)
        } else {
            UnitQuaternion::identity()
        };

        let sound_speed = geometry.sound_speed;
        Ok(Self::transform(geometry, |tr| {
            let r = tr.position() - self.pos;
            let r = Vector3::new(r.x, r.y, r.z);
            let r = rot * r;
            let dist = self.theta.sin() * (r.x * r.x + r.y * r.y).sqrt() - self.theta.cos() * r.z;
            let phase = dist * tr.wavenumber(sound_speed);
            Drive {
                phase,
                amp: self.amp,
            }
        }))
    }
}
