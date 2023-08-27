/*
 * File: trans_test.rs
 * Project: gain
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::{Gain, GainFilter},
    geometry::{Geometry, Transducer},
    Drive,
};

use autd3_derive::Gain;

/// Gain to drive only specified transducers
#[derive(Gain, Default, Clone)]
pub struct TransducerTest {
    test_drive: HashMap<usize, (float, float)>,
}

impl TransducerTest {
    /// constructor
    pub fn new() -> Self {
        Self {
            test_drive: HashMap::new(),
        }
    }

    /// set drive
    ///
    /// # Arguments
    ///
    /// * `id` - transducer id
    /// * `phase` - phase
    /// * `amp` - normalized amplitude (from 0 to 1)
    pub fn set(mut self, id: usize, phase: float, amp: float) -> Self {
        self.test_drive.insert(id, (phase, amp));
        self
    }

    /// get drive map which maps transducer id to phase and amplitude
    pub fn test_drive(&self) -> &HashMap<usize, (float, float)> {
        &self.test_drive
    }
}

impl<T: Transducer> Gain<T> for TransducerTest {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |tr| {
            if let Some(&(phase, amp)) = self.test_drive.get(&tr.idx()) {
                Drive { phase, amp }
            } else {
                Drive {
                    phase: 0.0,
                    amp: 0.0,
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use autd3_core::autd3_device::AUTD3;
    use autd3_core::geometry::{LegacyTransducer, Vector3};
    use rand::Rng;

    use super::*;

    use crate::tests::GeometryBuilder;

    #[test]
    fn test_transducer_test() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let mut transducer_test = TransducerTest::new();

        let mut rng = rand::thread_rng();
        let test_id = rng.gen_range(0..geometry.num_transducers());
        let test_phase = rng.gen_range(-1.0..1.0);
        let test_amp = rng.gen_range(-1.0..1.0);

        transducer_test = transducer_test.set(test_id, test_phase, test_amp);

        let drives = transducer_test.calc(&geometry, GainFilter::All).unwrap();

        drives.iter().enumerate().for_each(|(idx, drive)| {
            if idx == test_id {
                assert_eq!(drive.phase, test_phase);
                assert_eq!(drive.amp, test_amp);
            } else {
                assert_eq!(drive.phase, 0.0);
                assert_eq!(drive.amp, 0.0);
            }
        });
    }
}
