/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::Drive;

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    error::AUTDInternalError,
    geometry::{AdvancedPhaseTransducer, AdvancedTransducer, Geometry, LegacyTransducer},
};

#[derive(Default)]
pub struct Stop {}

impl Stop {
    pub fn new() -> Self {
        Self {}
    }
}

impl DatagramBody<LegacyTransducer> for Stop {
    type O = autd3_driver::GainLegacy;

    fn operation(
        &mut self,
        geometry: &Geometry<LegacyTransducer>,
    ) -> Result<Self::O, AUTDInternalError> {
        let drives = vec![Drive { amp: 0., phase: 0. }; geometry.num_transducers()];
        Ok(Self::O::new(drives))
    }
}

impl Sendable<LegacyTransducer> for Stop {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<LegacyTransducer>>::O;

    fn operation(
        &mut self,
        geometry: &Geometry<LegacyTransducer>,
    ) -> Result<Self::O, AUTDInternalError> {
        <Self as DatagramBody<LegacyTransducer>>::operation(self, geometry)
    }
}

impl DatagramBody<AdvancedTransducer> for Stop {
    type O = autd3_driver::GainAdvanced;

    fn operation(
        &mut self,
        geometry: &Geometry<AdvancedTransducer>,
    ) -> Result<Self::O, AUTDInternalError> {
        let drives = vec![Drive { amp: 0., phase: 0. }; geometry.num_transducers()];
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(drives, cycles))
    }
}

impl Sendable<AdvancedTransducer> for Stop {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<AdvancedTransducer>>::O;

    fn operation(
        &mut self,
        geometry: &Geometry<AdvancedTransducer>,
    ) -> Result<Self::O, AUTDInternalError> {
        <Self as DatagramBody<AdvancedTransducer>>::operation(self, geometry)
    }
}

impl DatagramBody<AdvancedPhaseTransducer> for Stop {
    type O = autd3_driver::GainAdvancedDuty;

    fn operation(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<Self::O, AUTDInternalError> {
        let drives = (0..geometry.num_transducers())
            .map(|_| Drive { phase: 0., amp: 0. })
            .collect();
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(drives, cycles))
    }
}

impl Sendable<AdvancedPhaseTransducer> for Stop {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<AdvancedPhaseTransducer>>::O;

    fn operation(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<Self::O, AUTDInternalError> {
        <Self as DatagramBody<AdvancedPhaseTransducer>>::operation(self, geometry)
    }
}
