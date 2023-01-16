/*
 * File: synchronize.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{Geometry, LegacyTransducer, NormalPhaseTransducer, NormalTransducer},
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

impl DatagramBody<NormalTransducer> for Synchronize {
    type O = autd3_driver::SyncNormal;

    fn operation(&mut self, geometry: &Geometry<NormalTransducer>) -> Result<Self::O> {
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(cycles))
    }
}

impl Sendable<NormalTransducer> for Synchronize {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<NormalTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<NormalTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<NormalTransducer>>::operation(self, geometry)
    }
}

impl DatagramBody<NormalPhaseTransducer> for Synchronize {
    type O = autd3_driver::SyncNormal;

    fn operation(&mut self, geometry: &Geometry<NormalPhaseTransducer>) -> Result<Self::O> {
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(cycles))
    }
}

impl Sendable<NormalPhaseTransducer> for Synchronize {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<NormalPhaseTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<NormalPhaseTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<NormalPhaseTransducer>>::operation(self, geometry)
    }
}
