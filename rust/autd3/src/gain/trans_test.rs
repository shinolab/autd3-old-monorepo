/*
 * File: trans_test.rs
 * Project: gain
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{
    gain::GainProps,
    geometry::{Geometry, Transducer},
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain, Default)]
pub struct TransducerTest {
    props: GainProps,
    test_drive: HashMap<usize, (f64, f64)>,
}

impl TransducerTest {
    /// constructor
    pub fn new() -> Self {
        Self {
            props: GainProps::default(),
            test_drive: HashMap::new(),
        }
    }

    pub fn set(&mut self, id: usize, phase: f64, amp: f64) {
        self.test_drive.insert(id, (phase, amp));
    }
}

impl TransducerTest {
    fn calc<T: Transducer>(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        geometry.transducers().for_each(|tr| {
            if let Some((phase, amp)) = self.test_drive.get(&tr.id()) {
                self.props.drives[tr.id()].amp = *amp;
                self.props.drives[tr.id()].phase = *phase;
            } else {
                self.props.drives[tr.id()].amp = 0.0;
                self.props.drives[tr.id()].phase = 0.0;
            }
        });

        Ok(())
    }
}
