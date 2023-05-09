/*
 * File: trans_test.rs
 * Project: gain
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{
    error::AUTDInternalError,
    gain::Gain,
    geometry::{Geometry, Transducer},
    Drive,
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain, Default)]
pub struct TransducerTest {
    test_drive: HashMap<usize, (f64, f64)>,
}

impl TransducerTest {
    /// constructor
    pub fn new() -> Self {
        Self {
            test_drive: HashMap::new(),
        }
    }

    pub fn set(&mut self, id: usize, phase: f64, amp: f64) {
        self.test_drive.insert(id, (phase, amp));
    }
}

impl<T: Transducer> Gain<T> for TransducerTest {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(geometry
            .transducers()
            .map(|tr| {
                if let Some(&(phase, amp)) = self.test_drive.get(&tr.idx()) {
                    Drive { phase, amp }
                } else {
                    Drive {
                        phase: 0.0,
                        amp: 0.0,
                    }
                }
            })
            .collect())
    }
}
