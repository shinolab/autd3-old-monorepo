/*
 * File: synchronize.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_driver::NullHeader;

use crate::{
    error::AUTDInternalError,
    geometry::{AdvancedPhaseTransducer, AdvancedTransducer, Geometry, LegacyTransducer},
    sendable::Sendable,
};

#[derive(Default)]
pub struct Synchronize {}

impl Synchronize {
    pub fn new() -> Self {
        Self {}
    }
}

impl Sendable<LegacyTransducer> for Synchronize {
    type H = NullHeader;
    type B = autd3_driver::SyncLegacy;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Default::default())
    }

    fn body_operation(
        &mut self,
        _geometry: &Geometry<LegacyTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::default())
    }

    fn timeout() -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}

impl Sendable<AdvancedTransducer> for Synchronize {
    type H = NullHeader;
    type B = autd3_driver::SyncAdvanced;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Default::default())
    }

    fn body_operation(
        &mut self,
        geometry: &Geometry<AdvancedTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(
            geometry.transducers().map(|tr| tr.cycle()).collect(),
        ))
    }

    fn timeout() -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}

impl Sendable<AdvancedPhaseTransducer> for Synchronize {
    type H = NullHeader;
    type B = autd3_driver::SyncAdvanced;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Default::default())
    }

    fn body_operation(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(
            geometry.transducers().map(|tr| tr.cycle()).collect(),
        ))
    }

    fn timeout() -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}
