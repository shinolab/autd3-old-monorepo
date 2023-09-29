/*
 * File: plane.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_derive::Gain;

use autd3_driver::{
    derive::prelude::*,
    geometry::{Geometry, Vector3},
};
/// Gain to produce a plane wave
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
    /// * `dir` - direction of the plane wave
    ///
    pub fn new(dir: Vector3) -> Self {
        Self { dir, amp: 1.0 }
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

    pub fn dir(&self) -> Vector3 {
        self.dir
    }
}

impl<T: Transducer> Gain<T> for Plane {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |dev, tr| {
            let dist = self.dir.dot(tr.position());
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

    use autd3_driver::geometry::{IntoDevice, LegacyTransducer};

    use super::*;

    use crate::{autd3_device::AUTD3, tests::random_vector3};

    #[test]
    fn test_plane() {
        let geometry: Geometry<LegacyTransducer> =
            Geometry::new(vec![
                AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0)
            ]);

        let d = random_vector3(-1.0..1.0, -1.0..1.0, -1.0..1.0).normalize();
        let p = Plane::new(d).calc(&geometry, GainFilter::All).unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[&0].len(), geometry.num_transducers());
        p[&0].iter().for_each(|d| assert_eq!(d.amp, 1.0));
        p[&0].iter().zip(geometry[0].iter()).for_each(|(p, tr)| {
            let expected_phase = d.dot(tr.position()) * tr.wavenumber(geometry[0].sound_speed);
            assert_approx_eq::assert_approx_eq!(p.phase, expected_phase);
        });

        let d = random_vector3(-1.0..1.0, -1.0..1.0, -1.0..1.0).normalize();
        let p = Plane::new(d)
            .with_amp(0.5)
            .calc(&geometry, GainFilter::All)
            .unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[&0].len(), geometry.num_transducers());
        p[&0].iter().for_each(|p| assert_eq!(p.amp, 0.5));
        p[&0].iter().zip(geometry[0].iter()).for_each(|(p, tr)| {
            let expected_phase = d.dot(tr.position()) * tr.wavenumber(geometry[0].sound_speed);
            assert_approx_eq::assert_approx_eq!(p.phase, expected_phase);
        });
    }
}
