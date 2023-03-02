/*
 * File: gain.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    gain::Gain,
    geometry::{
        AdvancedPhaseTransducer, AdvancedTransducer, Geometry, LegacyTransducer, Transducer,
    },
};

use anyhow::{Ok, Result};
use autd3_driver::{GainSTMProps, Mode};

use super::STM;

#[derive(Default)]
pub struct GainSTM<'a, T: Transducer> {
    gains: Vec<Box<dyn Gain<T> + 'a>>,
    props: GainSTMProps,
}

impl<'a, T: Transducer> GainSTM<'a, T> {
    pub fn new() -> Self {
        Self {
            gains: vec![],
            props: Default::default(),
        }
    }

    pub fn add<G: Gain<T> + 'a>(&mut self, gain: G) -> Result<()> {
        self.gains.push(Box::new(gain));
        Ok(())
    }
}

impl<'a, T: Transducer> STM for GainSTM<'a, T> {
    fn size(&self) -> usize {
        self.gains.len()
    }

    fn set_sampling_freq_div(&mut self, freq_div: u32) {
        self.props.freq_div = freq_div;
    }

    fn sampling_freq_div(&self) -> u32 {
        self.props.freq_div
    }

    fn set_start_idx(&mut self, idx: Option<u16>) {
        self.props.start_idx = idx
    }

    fn start_idx(&self) -> Option<u16> {
        self.props.start_idx
    }

    fn set_finish_idx(&mut self, idx: Option<u16>) {
        self.props.finish_idx = idx
    }

    fn finish_idx(&self) -> Option<u16> {
        self.props.finish_idx
    }
}

impl<'a> GainSTM<'a, LegacyTransducer> {
    pub fn mode(&self) -> Mode {
        self.props.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.props.mode = mode;
    }
}

impl<'a> GainSTM<'a, AdvancedTransducer> {
    pub fn mode(&self) -> Mode {
        self.props.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.props.mode = mode;
    }
}

impl<'a> DatagramBody<LegacyTransducer> for GainSTM<'a, LegacyTransducer> {
    type O = autd3_driver::GainSTMLegacy;

    fn operation(&mut self, geometry: &Geometry<LegacyTransducer>) -> Result<Self::O> {
        let drives = self
            .gains
            .iter_mut()
            .map(|g| g.calc(geometry))
            .collect::<Result<_>>()?;
        Ok(Self::O::new(drives, self.props))
    }
}

impl<'a> Sendable<LegacyTransducer> for GainSTM<'a, LegacyTransducer> {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<LegacyTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<LegacyTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<LegacyTransducer>>::operation(self, geometry)
    }
}

impl<'a> DatagramBody<AdvancedTransducer> for GainSTM<'a, AdvancedTransducer> {
    type O = autd3_driver::GainSTMAdvanced;

    fn operation(&mut self, geometry: &Geometry<AdvancedTransducer>) -> Result<Self::O> {
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        let drives = self
            .gains
            .iter_mut()
            .map(|g| g.calc(geometry))
            .collect::<Result<_>>()?;
        Ok(Self::O::new(drives, cycles, self.props))
    }
}

impl<'a> Sendable<AdvancedTransducer> for GainSTM<'a, AdvancedTransducer> {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<AdvancedTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<AdvancedTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<AdvancedTransducer>>::operation(self, geometry)
    }
}

impl<'a> DatagramBody<AdvancedPhaseTransducer> for GainSTM<'a, AdvancedPhaseTransducer> {
    type O = autd3_driver::GainSTMAdvanced;

    fn operation(&mut self, geometry: &Geometry<AdvancedPhaseTransducer>) -> Result<Self::O> {
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        let drives = self
            .gains
            .iter_mut()
            .map(|g| g.calc(geometry))
            .collect::<Result<_>>()?;
        Ok(Self::O::new(drives, cycles, self.props))
    }
}

impl<'a> Sendable<AdvancedPhaseTransducer> for GainSTM<'a, AdvancedPhaseTransducer> {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<AdvancedPhaseTransducer>>::O;

    fn operation(&mut self, geometry: &Geometry<AdvancedPhaseTransducer>) -> Result<Self::O> {
        <Self as DatagramBody<AdvancedPhaseTransducer>>::operation(self, geometry)
    }
}
