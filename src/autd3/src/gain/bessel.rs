/*
 * File: bessel.rs
 * Project: gain
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/07/2023
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

/// Gain to produce a Bessel beam
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
    /// * `pos` - Start point of the beam (the apex of the conical wavefront of the beam)
    /// * `dir` - Direction of the beam
    /// * `theta` - Angle between the conical wavefront of the beam and the plane normal to `dir`
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

    pub fn amp(&self) -> float {
        self.amp
    }

    pub fn pos(&self) -> Vector3 {
        self.pos
    }

    pub fn dir(&self) -> Vector3 {
        self.dir
    }

    pub fn theta(&self) -> float {
        self.theta
    }
}

impl<T: Transducer> Gain<T> for Bessel {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
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

#[cfg(test)]
mod tests {
    use rand::Rng;

    use autd3_core::{autd3_device::AUTD3, geometry::LegacyTransducer, PI};

    use super::*;

    use crate::tests::{random_vector3, GeometryBuilder};

    #[test]
    fn test_bessel() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let f = random_vector3(-500.0..500.0, -500.0..500.0, 50.0..500.0);
        let d = random_vector3(-1.0..1.0, -1.0..1.0, -1.0..1.0).normalize();
        let mut rng = rand::thread_rng();
        let theta = rng.gen_range(-PI..PI);
        let b = Bessel::new(f, d, theta).calc(&geometry).unwrap();
        assert_eq!(b.len(), geometry.num_transducers());
        b.iter().for_each(|d| assert_eq!(d.amp, 1.0));
        b.iter().zip(geometry.iter()).for_each(|(b, tr)| {
            let expected_phase = {
                let dir = d;
                let v = Vector3::new(dir.y, -dir.x, 0.);
                let theta_v = v.norm().asin();
                let rot = if let Some(v) = v.try_normalize(1.0e-6) {
                    UnitQuaternion::from_scaled_axis(v * -theta_v)
                } else {
                    UnitQuaternion::identity()
                };
                let r = tr.position() - f;
                let r = rot * r;
                let dist = theta.sin() * (r.x * r.x + r.y * r.y).sqrt() - theta.cos() * r.z;
                dist * tr.wavenumber(geometry.sound_speed)
            };
            assert_approx_eq::assert_approx_eq!(b.phase, expected_phase);
        });

        let f = random_vector3(-500.0..500.0, -500.0..500.0, 50.0..500.0);
        let d = random_vector3(-1.0..1.0, -1.0..1.0, -1.0..1.0).normalize();
        let theta = rng.gen_range(-PI..PI);
        let b = Bessel::new(f, d, theta)
            .with_amp(0.5)
            .calc(&geometry)
            .unwrap();
        assert_eq!(b.len(), geometry.num_transducers());
        b.iter().for_each(|b| assert_eq!(b.amp, 0.5));
        b.iter().zip(geometry.iter()).for_each(|(b, tr)| {
            let expected_phase = {
                let dir = d;
                let v = Vector3::new(dir.y, -dir.x, 0.);
                let theta_v = v.norm().asin();
                let rot = if let Some(v) = v.try_normalize(1.0e-6) {
                    UnitQuaternion::from_scaled_axis(v * -theta_v)
                } else {
                    UnitQuaternion::identity()
                };
                let r = tr.position() - f;
                let r = rot * r;
                let dist = theta.sin() * (r.x * r.x + r.y * r.y).sqrt() - theta.cos() * r.z;
                dist * tr.wavenumber(geometry.sound_speed)
            };
            assert_approx_eq::assert_approx_eq!(b.phase, expected_phase);
        });
    }
}
