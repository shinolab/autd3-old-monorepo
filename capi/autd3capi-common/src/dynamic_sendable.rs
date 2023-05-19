/*
 * File: dynamic_sendable.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3::{
    core::{
        error::AUTDInternalError,
        gain::Gain,
        modulation::{Modulation, ModulationProperty},
        sendable::{NullBody, NullHeader, Sendable},
        Drive, GainSTMProps, Operation,
    },
    prelude::*,
};

use crate::dynamic_transducer::{DynamicTransducer, TransMode};

pub trait DynamicSendable {
    fn operation(
        &mut self,
        mode: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<(Box<dyn Operation>, Box<dyn Operation>), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl Sendable<DynamicTransducer> for (TransMode, &mut Box<dyn DynamicSendable>) {
    type H = Box<dyn Operation>;
    type B = Box<dyn Operation>;

    fn operation(
        &mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let mode = self.0;
        (**self.1).operation(mode, geometry)
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
                Box::new(autd3::core::NullHeader::default()),
                Box::new(autd3::core::SyncLegacy::default()),
            )),
            TransMode::Advanced => Ok((
                Box::new(autd3::core::NullHeader::default()),
                Box::new(autd3::core::SyncAdvanced::new(
                    geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
            TransMode::AdvancedPhase => Ok((
                Box::new(autd3::core::NullHeader::default()),
                Box::new(autd3::core::SyncAdvanced::new(
                    geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
        }
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
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
        Some(Duration::from_millis(200))
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
        match mode {
            TransMode::Legacy => {
                let drives = self
                    .gains_mut()
                    .iter_mut()
                    .map(|gain| gain.calc(geometry))
                    .collect::<Result<Vec<_>, _>>()?;
                let props = GainSTMProps {
                    mode: self.mode(),
                    freq_div: self.sampling_freq_div(),
                    finish_idx: self.finish_idx(),
                    start_idx: self.start_idx(),
                };
                Ok((
                    Box::new(autd3::core::NullHeader::default()),
                    Box::new(autd3::core::GainSTMLegacy::new(drives, props)),
                ))
            }
            TransMode::Advanced => {
                let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
                let drives = self
                    .gains_mut()
                    .iter_mut()
                    .map(|gain| gain.calc(geometry))
                    .collect::<Result<Vec<_>, _>>()?;
                let props = GainSTMProps {
                    mode: self.mode(),
                    freq_div: self.sampling_freq_div(),
                    finish_idx: self.finish_idx(),
                    start_idx: self.start_idx(),
                };
                Ok((
                    Box::new(autd3::core::NullHeader::default()),
                    Box::new(autd3::core::GainSTMAdvanced::new(drives, cycles, props)),
                ))
            }
            TransMode::AdvancedPhase => {
                let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
                let drives = self
                    .gains_mut()
                    .iter_mut()
                    .map(|gain| gain.calc(geometry))
                    .collect::<Result<Vec<_>, _>>()?;
                let props = GainSTMProps {
                    mode: self.mode(),
                    freq_div: self.sampling_freq_div(),
                    finish_idx: self.finish_idx(),
                    start_idx: self.start_idx(),
                };
                Ok((
                    Box::new(autd3::core::NullHeader::default()),
                    Box::new(autd3::core::GainSTMAdvanced::new(drives, cycles, props)),
                ))
            }
        }
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
            Box::new(autd3::core::NullHeader::default()),
            Box::new(autd3::core::GainAdvancedDuty::new(
                vec![
                    Drive {
                        phase: 0.0,
                        amp: self.amp(),
                    };
                    geometry.num_transducers()
                ],
                geometry.transducers().map(|tr| tr.cycle()).collect(),
            )),
        ))
    }
}

#[macro_export]
macro_rules! impl_sendable_for_gain {
    ($g:ident) => {
        impl DynamicSendable for $g {
            fn operation(
                &mut self,
                mode: TransMode,
                geometry: &Geometry<DynamicTransducer>,
            ) -> Result<
                (
                    Box<dyn autd3::core::Operation>,
                    Box<dyn autd3::core::Operation>,
                ),
                autd3::core::error::AUTDInternalError,
            > {
                match mode {
                    TransMode::Legacy => Ok((
                        Box::new(autd3::core::NullHeader::default()),
                        Box::new(autd3::core::GainLegacy::new(self.calc(geometry)?)),
                    )),
                    TransMode::Advanced => Ok((
                        Box::new(autd3::core::NullHeader::default()),
                        Box::new(autd3::core::GainAdvanced::new(
                            self.calc(geometry)?,
                            geometry.transducers().map(|tr| tr.cycle()).collect(),
                        )),
                    )),
                    TransMode::AdvancedPhase => Ok((
                        Box::new(autd3::core::NullHeader::default()),
                        Box::new(autd3::core::GainAdvancedPhase::new(
                            self.calc(geometry)?,
                            geometry.transducers().map(|tr| tr.cycle()).collect(),
                        )),
                    )),
                }
            }
        }
    };
}

impl_sendable_for_gain!(Null);
impl_sendable_for_gain!(Focus);
impl_sendable_for_gain!(Bessel);
impl_sendable_for_gain!(Plane);
impl_sendable_for_gain!(TransducerTest);
impl<'a> DynamicSendable for Grouped<'a, DynamicTransducer> {
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
                Box::new(autd3::core::NullHeader::default()),
                Box::new(autd3::core::GainLegacy::new(self.calc(geometry)?)),
            )),
            TransMode::Advanced => Ok((
                Box::new(autd3::core::NullHeader::default()),
                Box::new(autd3::core::GainAdvanced::new(
                    self.calc(geometry)?,
                    geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
            TransMode::AdvancedPhase => Ok((
                Box::new(autd3::core::NullHeader::default()),
                Box::new(autd3::core::GainAdvancedPhase::new(
                    self.calc(geometry)?,
                    geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
        }
    }
}

#[macro_export]
macro_rules! impl_sendable_for_modulation {
    ($m:ident) => {
        impl DynamicSendable for $m {
            fn operation(
                &mut self,
                _: TransMode,
                _: &Geometry<DynamicTransducer>,
            ) -> Result<
                (
                    Box<dyn autd3::core::Operation>,
                    Box<dyn autd3::core::Operation>,
                ),
                autd3::core::error::AUTDInternalError,
            > {
                let freq_div = self.sampling_frequency_division();
                Ok((
                    Box::new(autd3::core::Modulation::new(self.calc()?, freq_div)),
                    Box::new(autd3::core::NullBody::default()),
                ))
            }
        }
    };
}

impl_sendable_for_modulation!(Static);
impl_sendable_for_modulation!(Sine);
impl_sendable_for_modulation!(SinePressure);
impl_sendable_for_modulation!(SineLegacy);
impl_sendable_for_modulation!(Square);
