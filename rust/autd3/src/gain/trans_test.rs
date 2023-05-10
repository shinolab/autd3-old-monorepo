/*
 * File: trans_test.rs
 * Project: gain
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer},
    Drive,
};

use autd3_traits::Gain;

/// Gain to produce single focal point
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

    pub fn set(&mut self, id: usize, phase: float, amp: float) {
        self.test_drive.insert(id, (phase, amp));
    }
}

impl<T: Transducer> Gain<T> for TransducerTest {
    fn calc(self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(Self::transform(geometry, |tr| {
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
