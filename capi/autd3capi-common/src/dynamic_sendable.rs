/*
 * File: dynamic_sendable.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3::{
    core::{
        error::AUTDInternalError,
        sendable::{NullBody, NullHeader, Sendable},
        Drive, GainOp, GainSTMOp, GainSTMProps, Operation, SyncOp,
    },
    prelude::*,
};

use crate::dynamic_transducer::{DynamicTransducer, TransMode};

pub trait DynamicSendable {
    #[allow(clippy::type_complexity)]
    fn operation(
        &mut self,
        mode: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration>;
}

impl Sendable<DynamicTransducer> for (TransMode, &mut Box<dyn DynamicSendable>) {
    type H = Box<dyn Operation>;
    type B = Box<dyn Operation>;

    fn operation(
        &mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let mode = self.0;
        self.1.operation(mode, geometry)
    }

    fn timeout(&self) -> Option<Duration> {
        self.1.timeout()
    }
}

impl Sendable<DynamicTransducer>
    for (
        TransMode,
        &mut Box<dyn DynamicSendable>,
        &mut Box<dyn DynamicSendable>,
    )
{
    type H = Box<dyn Operation>;
    type B = Box<dyn Operation>;

    fn operation(
        &mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let mode = self.0;
        let (h, _) = (**self.1).operation(mode, geometry)?;
        let (_, b) = (**self.2).operation(mode, geometry)?;
        Ok((h, b))
    }
}

impl DynamicSendable for NullHeader {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let (h, b) = <Self as Sendable<DynamicTransducer>>::operation(self, geometry)?;
        Ok((Box::new(h), Box::new(b)))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicSendable for NullBody {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError> {
        let (h, b) = <Self as Sendable<DynamicTransducer>>::operation(self, geometry)?;
        Ok((Box::new(h), Box::new(b)))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicSendable for UpdateFlag {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        let (h, b) = <Self as Sendable<DynamicTransducer>>::operation(self, geometry)?;
        Ok((Box::new(h), Box::new(b)))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicSendable for Synchronize {
    fn operation(
        &mut self,
        mode: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        match mode {
            TransMode::Legacy => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::SyncLegacy::new(|| {
                    geometry.transducers().map(|tr| tr.cycle()).collect()
                })),
            )),
            TransMode::Advanced => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::SyncAdvanced::new(|| {
                    geometry.transducers().map(|tr| tr.cycle()).collect()
                })),
            )),
            TransMode::AdvancedPhase => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::SyncAdvanced::new(|| {
                    geometry.transducers().map(|tr| tr.cycle()).collect()
                })),
            )),
        }
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<LegacyTransducer>>::timeout(self)
    }
}

impl DynamicSendable for Stop {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        let (h, b) = <Self as Sendable<DynamicTransducer>>::operation(self, geometry)?;
        Ok((Box::new(h), Box::new(b)))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicSendable for SilencerConfig {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        let (h, b) = <Self as Sendable<DynamicTransducer>>::operation(self, geometry)?;
        Ok((Box::new(h), Box::new(b)))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicSendable for Clear {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        let (h, b) = <Self as Sendable<DynamicTransducer>>::operation(self, geometry)?;
        Ok((Box::new(h), Box::new(b)))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicSendable for ModDelay {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        let (h, b) = <Self as Sendable<DynamicTransducer>>::operation(self, geometry)?;
        Ok((Box::new(h), Box::new(b)))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<DynamicTransducer>>::timeout(self)
    }
}

impl DynamicSendable for FocusSTM {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        let (h, b) = <Self as Sendable<DynamicTransducer>>::operation(self, geometry)?;
        Ok((Box::new(h), Box::new(b)))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<DynamicTransducer>>::timeout(self)
    }
}

impl<'a> DynamicSendable for GainSTM<'a, DynamicTransducer> {
    fn operation(
        &mut self,
        mode: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        let props = GainSTMProps {
            mode: self.mode(),
            freq_div: self.sampling_freq_div(),
            finish_idx: self.finish_idx(),
            start_idx: self.start_idx(),
        };
        let drives = self
            .gains_mut()
            .iter_mut()
            .map(|gain| gain.calc(geometry))
            .collect::<Result<Vec<_>, _>>()?;
        match mode {
            TransMode::Legacy => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::GainSTMLegacy::new(drives, props, || {
                    geometry.transducers().map(|tr| tr.cycle()).collect()
                })),
            )),
            TransMode::Advanced => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::GainSTMAdvanced::new(drives, props, || {
                    geometry.transducers().map(|tr| tr.cycle()).collect()
                })),
            )),
            TransMode::AdvancedPhase => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::GainSTMAdvanced::new(drives, props, || {
                    geometry.transducers().map(|tr| tr.cycle()).collect()
                })),
            )),
        }
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl DynamicSendable for Amplitudes {
    fn operation(
        &mut self,
        _: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::<autd3::core::NullHeader>::default(),
            Box::new(autd3::core::GainAdvancedDuty::new(
                vec![
                    Drive {
                        phase: 0.0,
                        amp: self.amp(),
                    };
                    geometry.num_transducers()
                ],
                || geometry.transducers().map(|tr| tr.cycle()).collect(),
            )),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Sendable<AdvancedPhaseTransducer>>::timeout(self)
    }
}
