/*
 * File: null.rs
 * Project: gain
 * Created Date: 01/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    defined::Drive,
    error::AUTDInternalError,
    gain::{Gain, GainFilter},
    geometry::{Device, Transducer},
};

use autd3_derive::Gain;

/// Gain to output nothing
#[derive(Gain, Default, Clone, Copy)]
pub struct Null {}

impl Null {
    /// constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Gain<T> for Null {
    fn calc(
        &self,
        devices: &[&Device<T>],
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(devices, filter, |_, _| Drive {
            phase: 0.,
            amp: 0.,
        }))
    }
}

#[cfg(test)]
mod tests {

    use autd3_driver::geometry::{IntoDevice, LegacyTransducer, Vector3};

    use super::*;

    use crate::autd3_device::AUTD3;

    #[test]
    fn test_null() {
        let device: Device<LegacyTransducer> =
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0);

        let null_gain = Null::new();

        let drives = null_gain.calc(&[&device], GainFilter::All).unwrap();
        assert_eq!(drives.len(), 1);
        assert_eq!(drives[&0].len(), device.num_transducers());
        drives[&0].iter().for_each(|d| {
            assert_eq!(d.amp, 0.0);
            assert_eq!(d.phase, 0.0);
        });
    }
}
