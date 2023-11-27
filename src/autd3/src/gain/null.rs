/*
 * File: null.rs
 * Project: gain
 * Created Date: 01/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{common::EmitIntensity, derive::prelude::*, geometry::Geometry};

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

impl Gain for Null {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |_, _| Drive {
            phase: 0.,
            intensity: EmitIntensity::MIN,
        }))
    }
}

#[cfg(test)]
mod tests {

    use autd3_driver::{
        autd3_device::AUTD3,
        geometry::{IntoDevice, Vector3},
    };

    use super::*;

    #[test]
    fn test_null() {
        let geometry: Geometry = Geometry::new(vec![AUTD3::new(Vector3::zeros()).into_device(0)]);

        let null_gain = Null::new();

        let drives = null_gain.calc(&geometry, GainFilter::All).unwrap();
        assert_eq!(drives.len(), 1);
        assert_eq!(drives[&0].len(), geometry.num_transducers());
        drives[&0].iter().for_each(|d| {
            assert_eq!(d.intensity.value(), 0);
            assert_eq!(d.phase, 0.0);
        });
    }
}
