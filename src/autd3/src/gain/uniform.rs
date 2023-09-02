/*
 * File: uniform.rs
 * Project: gain
 * Created Date: 18/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_derive::Gain;
use autd3_driver::{
    defined::{float, Drive},
    error::AUTDInternalError,
    gain::{Gain, GainFilter},
    geometry::{Device, Transducer},
};

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
        devices: &[&Device<T>],
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(devices, filter, |_, _| Drive {
            phase: self.phase,
            amp: self.amp,
        }))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use autd3_driver::geometry::{IntoDevice, LegacyTransducer, Vector3};

    use crate::autd3_device::AUTD3;

    #[test]
    fn test_uniform() {
        let device: Device<LegacyTransducer> =
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0);

        let gain = Uniform::new(0.5);

        let d = gain.calc(&[&device], GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp, 0.5);
        });

        let gain = gain.with_phase(0.2);

        let d = gain.calc(&[&device], GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 0.2);
            assert_eq!(drive.amp, 0.5);
        });
    }
}
