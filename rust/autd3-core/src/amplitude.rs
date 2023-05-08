/*
 * File: amplitude.rs
 * Project: src
 * Created Date: 07/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{float, Drive};

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    error::AUTDInternalError,
    geometry::{AdvancedPhaseTransducer, Geometry},
};

pub struct Amplitudes {
    amp: float,
}

impl Amplitudes {
    pub fn uniform(amp: float) -> Self {
        Self { amp }
    }

    pub fn none() -> Self {
        Self::uniform(0.0)
    }
}

impl DatagramBody<AdvancedPhaseTransducer> for Amplitudes {
    type O = autd3_driver::GainAdvancedDuty;

    fn operation(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<Self::O, AUTDInternalError> {
        let drives = vec![
            Drive {
                phase: 0.0,
                amp: self.amp,
            };
            geometry.num_transducers()
        ];
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(drives, cycles))
    }
}

impl Sendable<AdvancedPhaseTransducer> for Amplitudes {
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
