/*
 * File: uniform.rs
 * Project: gain
 * Created Date: 18/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_derive::Gain;

use autd3_driver::{common::EmitIntensity, derive::prelude::*, geometry::Geometry};

/// Gain with uniform emission intensity and phase
#[derive(Gain, Clone, Copy)]
pub struct Uniform {
    intensity: EmitIntensity,
    phase: Phase,
}

impl Uniform {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `intensity` - normalized intensity (from 0 to 1)
    ///
    pub fn new<A: Into<EmitIntensity>>(intensity: A) -> Self {
        Self {
            intensity: intensity.into(),
            phase: Phase::new(0),
        }
    }

    /// set phase
    ///
    /// # Arguments
    ///
    /// * `phase` - phase
    ///
    pub fn with_phase(self, phase: Phase) -> Self {
        Self { phase, ..self }
    }
}

impl Gain for Uniform {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |_, _| Drive {
            phase: self.phase,
            intensity: self.intensity,
        }))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use autd3_driver::{
        autd3_device::AUTD3,
        geometry::{IntoDevice, Vector3},
    };

    #[test]
    fn test_uniform() {
        let geometry: Geometry = Geometry::new(vec![AUTD3::new(Vector3::zeros()).into_device(0)]);

        let gain = Uniform::new(0x1F);

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase.value(), 0);
            assert_eq!(drive.intensity.value(), 0x1F);
        });

        let gain = gain.with_phase(Phase::new(1));

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase.value(), 1);
            assert_eq!(drive.intensity.value(), 0x1F);
        });
    }
}
