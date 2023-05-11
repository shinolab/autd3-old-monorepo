/*
 * File: synchronize.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_driver::NullHeader;

use crate::{error::AUTDInternalError, geometry::*, sendable::Sendable};

#[derive(Default)]
pub struct Synchronize {}

impl Synchronize {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(not(feature = "dynamic"))]
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

#[cfg(not(feature = "dynamic"))]
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

#[cfg(not(feature = "dynamic"))]
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

#[cfg(feature = "dynamic")]
impl Sendable for Synchronize {
    fn operation(
        &mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3_driver::Operation>,
            Box<dyn autd3_driver::Operation>,
        ),
        AUTDInternalError,
    > {
        match geometry.mode() {
            TransMode::Legacy => Ok((
                Box::new(NullHeader::default()),
                Box::new(autd3_driver::SyncLegacy::default()),
            )),
            TransMode::Advanced => Ok((
                Box::new(NullHeader::default()),
                Box::new(autd3_driver::SyncAdvanced::new(
                    geometry
                        .transducers()
                        .map(|tr| tr.cycle().unwrap())
                        .collect(),
                )),
            )),
            TransMode::AdvancedPhase => Ok((
                Box::new(NullHeader::default()),
                Box::new(autd3_driver::SyncAdvanced::new(
                    geometry
                        .transducers()
                        .map(|tr| tr.cycle().unwrap())
                        .collect(),
                )),
            )),
        }
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}
