/*
 * File: uniform.rs
 * Project: gain
 * Created Date: 18/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::{Gain, GainFilter},
    geometry::{Geometry, Transducer},
    Drive,
};

use autd3_derive::Gain;

/// Gain with uniform amplitude and phase
#[derive(Gain, Default, Clone, Copy)]
pub struct Uniform {
    amp: float,
    phase: float,
}

impl Uniform {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `amp` - normalized amp (from 0 to 1)
    ///
    pub fn new(amp: float) -> Self {
        Self { amp, phase: 0. }
    }

    /// set phase
    ///
    /// # Arguments
    ///
    /// * `phase` - phasae
    ///
    pub fn with_phase(self, phase: float) -> Self {
        Self { phase, ..self }
    }
}

impl<T: Transducer> Gain<T> for Uniform {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |_| Drive {
            phase: self.phase,
            amp: self.amp,
        }))
    }
}

#[cfg(test)]
mod tests {
    use autd3_core::autd3_device::AUTD3;
    use autd3_core::geometry::{LegacyTransducer, Vector3};

    use super::*;

    use crate::tests::GeometryBuilder;

    #[test]
    fn test_uniform() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let gain = Uniform::new(0.5);

        for drive in gain.calc(&geometry, GainFilter::All).unwrap() {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp, 0.5);
        }

        let gain = gain.with_phase(0.2);

        for drive in gain.calc(&geometry, GainFilter::All).unwrap() {
            assert_eq!(drive.phase, 0.2);
            assert_eq!(drive.amp, 0.5);
        }
    }
}
