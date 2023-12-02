/*
 * File: trans_test.rs
 * Project: gain
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
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
    geometry::{Device, Geometry},
};

/// Gain to drive only specified transducers
#[derive(Gain, Default, Clone)]
pub struct TransducerTest<F: Fn(&Device, &Transducer) -> Option<Drive> + Sync + 'static> {
    f: F,
}

impl<F: Fn(&Device, &Transducer) -> Option<Drive> + Sync + 'static> TransducerTest<F> {
    /// constructor
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F: Fn(&Device, &Transducer) -> Option<Drive> + Sync + 'static> Gain for TransducerTest<F> {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |dev, tr| {
            if let Some(d) = (self.f)(dev, tr) {
                d
            } else {
                Drive {
                    phase: Phase::new(0),
                    intensity: EmitIntensity::MIN,
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use autd3_driver::{
        autd3_device::AUTD3,
        geometry::{IntoDevice, Vector3},
    };

    use super::*;

    #[test]
    fn test_transducer_test() {
        let geometry: Geometry = Geometry::new(vec![AUTD3::new(Vector3::zeros()).into_device(0)]);

        let mut rng = rand::thread_rng();
        let test_id = rng.gen_range(0..geometry.num_transducers());
        let test_phase = Phase::new(rng.gen_range(0x00..=0xFF));
        let test_intensity = EmitIntensity::new(rng.gen::<u8>());
        let transducer_test = TransducerTest::new(move |dev, tr| {
            if (dev.idx() == 0) && (tr.idx() == test_id) {
                Some(Drive {
                    phase: test_phase,
                    intensity: test_intensity,
                })
            } else {
                None
            }
        });

        let drives = transducer_test.calc(&geometry, GainFilter::All).unwrap();

        drives[&0].iter().enumerate().for_each(|(idx, drive)| {
            if idx == test_id {
                assert_eq!(drive.phase, test_phase);
                assert_eq!(drive.intensity, test_intensity);
            } else {
                assert_eq!(drive.phase.value(), 0);
                assert_eq!(drive.intensity.value(), 0);
            }
        });
    }
}
