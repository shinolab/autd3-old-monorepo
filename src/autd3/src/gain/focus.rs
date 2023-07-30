/*
 * File: focus.rs
 * Project: gain
 * Created Date: 28/04/2022
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
    geometry::{Geometry, Transducer, Vector3},
    Drive,
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain, Clone, Copy)]
pub struct Focus {
    amp: float,
    pos: Vector3,
}

impl Focus {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `pos` - position of focal point
    ///
    pub fn new(pos: Vector3) -> Self {
        Self { pos, amp: 1.0 }
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
}

impl<T: Transducer> Gain<T> for Focus {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let sound_speed = geometry.sound_speed;
        Ok(Self::transform(geometry, |tr| {
            let phase = tr.align_phase_at(self.pos, sound_speed);
            Drive {
                phase,
                amp: self.amp,
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use autd3_core::{
        acoustics::{propagate, Complex, Sphere},
        autd3_device::AUTD3,
        geometry::LegacyTransducer,
    };

    use super::*;

    use crate::tests::{random_vector3, GeometryBuilder};

    #[test]
    fn test_focus() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let f = random_vector3(-500.0..500.0, -500.0..500.0, 50.0..500.0);
        let d = Focus::new(f).calc(&geometry).unwrap();
        assert_eq!(d.len(), geometry.num_transducers());
        d.iter().for_each(|d| assert_eq!(d.amp, 1.0));
        d.iter().zip(geometry.iter()).for_each(|(d, tr)| {
            assert_approx_eq::assert_approx_eq!(
                (propagate::<Sphere>(
                    tr.position(),
                    &tr.z_direction(),
                    0.,
                    tr.wavenumber(geometry.sound_speed),
                    &f,
                ) * Complex::new(0., d.phase).exp())
                .arg(),
                0.
            )
        });

        let f = random_vector3(-500.0..500.0, -500.0..500.0, 50.0..500.0);
        let d = Focus::new(f).with_amp(0.5).calc(&geometry).unwrap();
        assert_eq!(d.len(), geometry.num_transducers());
        d.iter().for_each(|d| assert_eq!(d.amp, 0.5));
        d.iter().zip(geometry.iter()).for_each(|(d, tr)| {
            assert_approx_eq::assert_approx_eq!(
                (propagate::<Sphere>(
                    tr.position(),
                    &tr.z_direction(),
                    0.,
                    tr.wavenumber(geometry.sound_speed),
                    &f,
                ) * Complex::new(0., d.phase).exp())
                .arg(),
                0.
            )
        });
    }
}
