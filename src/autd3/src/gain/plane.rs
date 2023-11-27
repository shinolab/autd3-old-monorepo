/*
 * File: plane.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_derive::Gain;

use autd3_driver::{
    common::EmitIntensity,
    derive::prelude::*,
    geometry::{Geometry, Vector3},
};
/// Gain to produce a plane wave
#[derive(Gain, Clone, Copy)]
pub struct Plane {
    intensity: EmitIntensity,
    dir: Vector3,
}

impl Plane {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `dir` - direction of the plane wave
    ///
    pub fn new(dir: Vector3) -> Self {
        Self {
            dir,
            intensity: EmitIntensity::MAX,
        }
    }

    /// set emission intensity
    ///
    /// # Arguments
    ///
    /// * `intensity` - emission intensity
    ///
    pub fn with_intensity<A: Into<EmitIntensity>>(self, intensity: A) -> Self {
        Self {
            intensity: intensity.into(),
            ..self
        }
    }

    pub fn intensity(&self) -> EmitIntensity {
        self.intensity
    }

    pub fn dir(&self) -> Vector3 {
        self.dir
    }
}

impl Gain for Plane {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |dev, tr| {
            let dist = self.dir.dot(tr.position());
            let phase = dist * tr.wavenumber(dev.sound_speed);
            Drive {
                phase,
                intensity: self.intensity,
            }
        }))
    }
}

#[cfg(test)]
mod tests {

    use autd3_driver::{autd3_device::AUTD3, geometry::IntoDevice};

    use super::*;

    use crate::tests::random_vector3;

    #[test]
    fn test_plane() {
        let geometry: Geometry = Geometry::new(vec![AUTD3::new(Vector3::zeros()).into_device(0)]);

        let d = random_vector3(-1.0..1.0, -1.0..1.0, -1.0..1.0).normalize();
        let p = Plane::new(d).calc(&geometry, GainFilter::All).unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[&0].len(), geometry.num_transducers());
        p[&0]
            .iter()
            .for_each(|d| assert_eq!(d.intensity.value(), 0xFF));
        p[&0].iter().zip(geometry[0].iter()).for_each(|(p, tr)| {
            let expected_phase = d.dot(tr.position()) * tr.wavenumber(geometry[0].sound_speed);
            assert_approx_eq::assert_approx_eq!(p.phase, expected_phase);
        });

        let d = random_vector3(-1.0..1.0, -1.0..1.0, -1.0..1.0).normalize();
        let p = Plane::new(d)
            .with_intensity(0x1F)
            .calc(&geometry, GainFilter::All)
            .unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[&0].len(), geometry.num_transducers());
        p[&0]
            .iter()
            .for_each(|p| assert_eq!(p.intensity.value(), 0x1F));
        p[&0].iter().zip(geometry[0].iter()).for_each(|(p, tr)| {
            let expected_phase = d.dot(tr.position()) * tr.wavenumber(geometry[0].sound_speed);
            assert_approx_eq::assert_approx_eq!(p.phase, expected_phase);
        });
    }
}
