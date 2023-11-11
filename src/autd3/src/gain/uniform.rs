/*
 * File: uniform.rs
 * Project: gain
 * Created Date: 18/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_derive::Gain;

use autd3_driver::{
    common::{EmitIntensity, TryIntoEmitIntensity},
    derive::prelude::*,
    geometry::Geometry,
};

/// Gain with uniform amplitude and phase
#[derive(Gain, Clone, Copy)]
pub struct Uniform {
    amp: EmitIntensity,
    phase: float,
}

impl Uniform {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `amp` - normalized amp (from 0 to 1)
    ///
    pub fn new<A: TryIntoEmitIntensity>(amp: A) -> Result<Self, AUTDInternalError> {
        Ok(Self {
            amp: amp.try_into()?,
            phase: 0.,
        })
    }

    /// set phase
    ///
    /// # Arguments
    ///
    /// * `phase` - phase (from 0 to 2π)
    ///
    pub fn with_phase(self, phase: float) -> Self {
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
            amp: self.amp,
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
        let geometry: Geometry =
            Geometry::new(vec![
                AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0)
            ]);

        let gain = Uniform::new(0x1F).unwrap();

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp.pulse_width(), 0x1F);
        });

        let gain = gain.with_phase(0.2);

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 0.2);
            assert_eq!(drive.amp.pulse_width(), 0x1F);
        });
    }
}
