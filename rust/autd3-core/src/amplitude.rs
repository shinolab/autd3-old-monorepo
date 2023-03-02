/*
 * File: amplitude.rs
 * Project: src
 * Created Date: 07/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;
use autd3_driver::{Amp, Drive, Phase};

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{AdvancedPhaseTransducer, Geometry},
};

pub struct Amplitudes {
    amp: f64,
}

impl Amplitudes {
    pub fn uniform(amp: f64) -> Self {
        Self { amp }
    }

    pub fn none() -> Self {
        Self::uniform(0.0)
    }
}

impl DatagramBody<AdvancedPhaseTransducer> for Amplitudes {
    type O = autd3_driver::GainAdvancedDuty;

    fn operation(&mut self, geometry: &Geometry<AdvancedPhaseTransducer>) -> Result<Self::O> {
        let drives = (0..geometry.num_transducers())
            .map(|_| Drive {
                phase: Phase::new(0.0),
                amp: Amp::new(self.amp),
            })
            .collect();
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(drives, cycles))
    }
}

impl Sendable<AdvancedPhaseTransducer> for Amplitudes {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<AdvancedPhaseTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<AdvancedPhaseTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<AdvancedPhaseTransducer>>::operation(self, geometry)
    }
}
