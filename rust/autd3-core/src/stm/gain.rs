/*
 * File: gain.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{error::AUTDInternalError, gain::Gain, geometry::*, sendable::Sendable};

use autd3_driver::{GainSTMProps, Mode};

use super::STM;

#[derive(Default)]
pub struct GainSTM<'a, T: Transducer> {
    gains: Vec<Box<dyn Gain<T> + 'a>>,
    mode: Mode,
    pub freq_div: u32,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
}

impl<'a, T: Transducer> GainSTM<'a, T> {
    pub fn new() -> Self {
        Self {
            gains: vec![],
            mode: Mode::PhaseDutyFull,
            freq_div: 4096,
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn add<G: Gain<T> + 'a>(&mut self, gain: G) {
        self.gains.push(Box::new(gain));
    }
}

impl<'a, T: Transducer> STM for GainSTM<'a, T> {
    fn size(&self) -> usize {
        self.gains.len()
    }

    fn set_sampling_freq_div(&mut self, freq_div: u32) {
        self.freq_div = freq_div;
    }

    fn sampling_freq_div(&self) -> u32 {
        self.freq_div
    }
}

#[cfg(not(feature = "dynamic"))]
impl<'a> Sendable<LegacyTransducer> for GainSTM<'a, LegacyTransducer> {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::GainSTMLegacy;

    fn operation(
        mut self,
        geometry: &Geometry<LegacyTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let mut drives = Vec::with_capacity(self.gains.len());
        for gain in &mut self.gains {
            let drive = gain.calc(geometry)?;
            drives.push(drive);
        }
        let props = GainSTMProps {
            mode: self.mode,
            freq_div: self.freq_div,
            finish_idx: self.finish_idx,
            start_idx: self.start_idx,
        };
        Ok((Self::H::default(), Self::B::new(drives, props)))
    }
}

#[cfg(not(feature = "dynamic"))]
impl<'a> Sendable<AdvancedTransducer> for GainSTM<'a, AdvancedTransducer> {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::GainSTMAdvanced;

    fn operation(
        mut self,
        geometry: &Geometry<AdvancedTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        let mut drives = Vec::with_capacity(self.gains.len());
        for gain in &mut self.gains {
            let drive = gain.calc(geometry)?;
            drives.push(drive);
        }
        let props = GainSTMProps {
            mode: self.mode,
            freq_div: self.freq_div,
            finish_idx: self.finish_idx,
            start_idx: self.start_idx,
        };
        Ok((Self::H::default(), Self::B::new(drives, cycles, props)))
    }
}

#[cfg(not(feature = "dynamic"))]
impl<'a> Sendable<AdvancedPhaseTransducer> for GainSTM<'a, AdvancedPhaseTransducer> {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::GainSTMAdvanced;

    fn operation(
        mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        let mut drives = Vec::with_capacity(self.gains.len());
        for gain in &mut self.gains {
            let drive = gain.calc(geometry)?;
            drives.push(drive);
        }
        let props = GainSTMProps {
            mode: self.mode,
            freq_div: self.freq_div,
            finish_idx: self.finish_idx,
            start_idx: self.start_idx,
        };
        Ok((Self::H::default(), Self::B::new(drives, cycles, props)))
    }
}

#[cfg(feature = "dynamic")]
impl<'a> Sendable for GainSTM<'a, DynamicTransducer> {
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
            TransMode::Legacy => {
                let mut drives = Vec::with_capacity(self.gains.len());
                for gain in &mut self.gains {
                    let drive = gain.calc(geometry)?;
                    drives.push(drive);
                }
                let props = GainSTMProps {
                    mode: self.mode,
                    freq_div: self.freq_div,
                    finish_idx: self.finish_idx,
                    start_idx: self.start_idx,
                };
                Ok((
                    Box::new(autd3_driver::NullHeader::default()),
                    Box::new(autd3_driver::GainSTMLegacy::new(drives, props)),
                ))
            }
            TransMode::Advanced => {
                let cycles = geometry
                    .transducers()
                    .map(|tr| tr.cycle().unwrap())
                    .collect();
                let mut drives = Vec::with_capacity(self.gains.len());
                for gain in &mut self.gains {
                    let drive = gain.calc(geometry)?;
                    drives.push(drive);
                }
                let props = GainSTMProps {
                    mode: self.mode,
                    freq_div: self.freq_div,
                    finish_idx: self.finish_idx,
                    start_idx: self.start_idx,
                };
                Ok((
                    Box::new(autd3_driver::NullHeader::default()),
                    Box::new(autd3_driver::GainSTMAdvanced::new(drives, cycles, props)),
                ))
            }
            TransMode::AdvancedPhase => {
                let cycles = geometry
                    .transducers()
                    .map(|tr| tr.cycle().unwrap())
                    .collect();
                let mut drives = Vec::with_capacity(self.gains.len());
                for gain in &mut self.gains {
                    let drive = gain.calc(geometry)?;
                    drives.push(drive);
                }
                let props = GainSTMProps {
                    mode: self.mode,
                    freq_div: self.freq_div,
                    finish_idx: self.finish_idx,
                    start_idx: self.start_idx,
                };
                Ok((
                    Box::new(autd3_driver::NullHeader::default()),
                    Box::new(autd3_driver::GainSTMAdvanced::new(drives, cycles, props)),
                ))
            }
        }
    }
}
