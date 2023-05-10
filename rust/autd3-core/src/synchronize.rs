/*
 * File: synchronize.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/05/2023
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

    fn operation(
        self,
        _: &Geometry<LegacyTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Default::default(), Self::B::default()))
    }

    fn timeout() -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}

impl Sendable<AdvancedTransducer> for Synchronize {
    type H = NullHeader;
    type B = autd3_driver::SyncAdvanced;

    fn operation(
        self,
        geometry: &Geometry<AdvancedTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::default(),
            Self::B::new(geometry.transducers().map(|tr| tr.cycle()).collect()),
        ))
    }

    fn timeout() -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}

impl Sendable<AdvancedPhaseTransducer> for Synchronize {
    type H = NullHeader;
    type B = autd3_driver::SyncAdvanced;

    fn operation(
        self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::default(),
            Self::B::new(geometry.transducers().map(|tr| tr.cycle()).collect()),
        ))
    }

    fn timeout() -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}
