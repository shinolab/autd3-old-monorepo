/*
 * File: synchronize.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{AdvancedPhaseTransducer, AdvancedTransducer, Geometry, LegacyTransducer},
};

#[derive(Default)]
pub struct Synchronize {}

impl Synchronize {
    pub fn new() -> Self {
        Self {}
    }
}

impl DatagramBody<LegacyTransducer> for Synchronize {
    type O = autd3_driver::SyncLegacy;

    fn operation(&mut self, _: &Geometry<LegacyTransducer>) -> Result<Self::O> {
        Ok(Default::default())
    }
}

impl Sendable<LegacyTransducer> for Synchronize {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<LegacyTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<LegacyTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<LegacyTransducer>>::operation(self, geometry)
    }
}

impl DatagramBody<AdvancedTransducer> for Synchronize {
    type O = autd3_driver::SyncAdvanced;

    fn operation(&mut self, geometry: &Geometry<AdvancedTransducer>) -> Result<Self::O> {
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(cycles))
    }
}

impl Sendable<AdvancedTransducer> for Synchronize {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<AdvancedTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<AdvancedTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<AdvancedTransducer>>::operation(self, geometry)
    }
}

impl DatagramBody<AdvancedPhaseTransducer> for Synchronize {
    type O = autd3_driver::SyncAdvanced;

    fn operation(&mut self, geometry: &Geometry<AdvancedPhaseTransducer>) -> Result<Self::O> {
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(cycles))
    }
}

impl Sendable<AdvancedPhaseTransducer> for Synchronize {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<AdvancedPhaseTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<AdvancedPhaseTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<AdvancedPhaseTransducer>>::operation(self, geometry)
    }
}
