/*
 * File: trans_test.rs
 * Project: gain
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_derive::Gain;

use autd3_driver::{
    common::{EmitIntensity, TryIntoEmitIntensity},
    derive::prelude::*,
    geometry::Geometry,
};

/// Gain to drive only specified transducers
#[derive(Gain, Default, Clone)]
pub struct TransducerTest {
    test_drive: HashMap<(usize, usize), (float, EmitIntensity)>,
}

impl TransducerTest {
    /// constructor
    pub fn new() -> Self {
        Self {
            test_drive: HashMap::new(),
        }
    }

    /// set drive parameters
    ///
    /// # Arguments
    ///
    /// * `dev_idx` - device transducer index
    /// * `tr_idx` - local transducer index
    /// * `phase` - phase (from 0 to 2π)
    /// * `amp` - normalized amplitude (from 0 to 1)
    pub fn set<A: TryIntoEmitIntensity>(
        mut self,
        dev_idx: usize,
        tr_idx: usize,
        phase: float,
        amp: A,
    ) -> Result<Self, AUTDInternalError> {
        self.test_drive
            .insert((dev_idx, tr_idx), (phase, amp.try_into()?));
        Ok(self)
    }

    /// get drive map which maps (device index, local transducer index) index to phase and amplitude
    pub fn test_drive(&self) -> &HashMap<(usize, usize), (float, EmitIntensity)> {
        &self.test_drive
    }
}

impl Gain for TransducerTest {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |dev, tr| {
            if let Some(&(phase, amp)) = self.test_drive.get(&(dev.idx(), tr.local_idx())) {
                Drive { phase, amp }
            } else {
                Drive {
                    phase: 0.0,
                    amp: EmitIntensity::MIN,
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
        let geometry: Geometry =
            Geometry::new(vec![
                AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0)
            ]);

        let mut transducer_test = TransducerTest::new();

        let mut rng = rand::thread_rng();
        let test_id = rng.gen_range(0..geometry.num_transducers());
        let test_phase = rng.gen_range(-1.0..1.0);
        let test_amp = EmitIntensity::new_normalized(rng.gen_range(0.0..1.0)).unwrap();

        transducer_test = transducer_test
            .set(0, test_id, test_phase, test_amp)
            .unwrap();

        let drives = transducer_test.calc(&geometry, GainFilter::All).unwrap();

        drives[&0].iter().enumerate().for_each(|(idx, drive)| {
            if idx == test_id {
                assert_eq!(drive.phase, test_phase);
                assert_eq!(drive.amp.normalized(), test_amp.normalized());
            } else {
                assert_eq!(drive.phase, 0.0);
                assert_eq!(drive.amp.normalized(), 0.0);
            }
        });
    }
}
