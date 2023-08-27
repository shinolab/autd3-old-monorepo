/*
 * File: null.rs
 * Project: gain
 * Created Date: 01/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    gain::{Gain, GainFilter},
    geometry::{Geometry, Transducer},
    Drive,
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
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |_| Drive {
            phase: 0.,
            amp: 0.,
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
    fn test_null() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let null_gain = Null::new();

        let drives = null_gain.calc(&geometry, GainFilter::All).unwrap();

        for drive in drives {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp, 0.0);
        }
    }
}
