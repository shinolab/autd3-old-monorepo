/*
 * File: bessel.rs
 * Project: gain
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    datagram::{Gain, GainFilter},
    defined::{float, Drive},
    error::AUTDInternalError,
    geometry::{Device, Transducer, UnitQuaternion, Vector3},
};

use autd3_derive::Gain;

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
    fn calc(
        &self,
        devices: &[&Device<T>],
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        let dir = self.dir.normalize();
        let v = Vector3::new(dir.y, -dir.x, 0.);
        let theta_v = v.norm().asin();
        let rot = if let Some(v) = v.try_normalize(1.0e-6) {
            UnitQuaternion::from_scaled_axis(v * -theta_v)
        } else {
            UnitQuaternion::identity()
        };
        Ok(Self::transform(devices, filter, |dev, tr| {
            let r = tr.position() - self.pos;
            let r = rot * r;
            let dist = self.theta.sin() * (r.x * r.x + r.y * r.y).sqrt() - self.theta.cos() * r.z;
            let phase = dist * tr.wavenumber(dev.sound_speed);
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

    use autd3_driver::{
        defined::PI,
        geometry::{IntoDevice, LegacyTransducer},
    };

    use super::*;

    use crate::{autd3_device::AUTD3, tests::random_vector3};

    #[test]
    fn test_bessel() {
        let device: Device<LegacyTransducer> =
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0);

        let f = random_vector3(-500.0..500.0, -500.0..500.0, 50.0..500.0);
        let d = random_vector3(-1.0..1.0, -1.0..1.0, -1.0..1.0).normalize();
        let mut rng = rand::thread_rng();
        let theta = rng.gen_range(-PI..PI);
        let b = Bessel::new(f, d, theta)
            .calc(&[&device], GainFilter::All)
            .unwrap();
        assert_eq!(b.len(), 1);
        assert_eq!(b[&0].len(), device.num_transducers());
        b[&0].iter().for_each(|d| assert_eq!(d.amp, 1.0));
        b[&0].iter().zip(device.iter()).for_each(|(b, tr)| {
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
                dist * tr.wavenumber(device.sound_speed)
            };
            assert_approx_eq::assert_approx_eq!(b.phase, expected_phase);
        });

        let f = random_vector3(-500.0..500.0, -500.0..500.0, 50.0..500.0);
        let d = random_vector3(-1.0..1.0, -1.0..1.0, -1.0..1.0).normalize();
        let theta = rng.gen_range(-PI..PI);
        let b = Bessel::new(f, d, theta)
            .with_amp(0.5)
            .calc(&[&device], GainFilter::All)
            .unwrap();
        assert_eq!(b.len(), 1);
        assert_eq!(b[&0].len(), device.num_transducers());
        b[&0].iter().for_each(|b| assert_eq!(b.amp, 0.5));
        b[&0].iter().zip(device.iter()).for_each(|(b, tr)| {
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
                dist * tr.wavenumber(device.sound_speed)
            };
            assert_approx_eq::assert_approx_eq!(b.phase, expected_phase);
        });
    }
}
