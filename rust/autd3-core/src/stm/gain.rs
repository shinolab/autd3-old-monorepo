/*
 * File: gain.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{error::AUTDInternalError, gain::Gain, geometry::*, sendable::*};

use autd3_driver::{GainSTMProps, Mode};

use super::STM;

#[derive(Default)]
pub struct GainSTM<'a, T: Transducer> {
    gains: Vec<Box<dyn Gain<T> + 'a>>,
    mode: Mode,
    freq_div: u32,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
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
}

impl<'a, T: Transducer> GainSTM<'a, T> {
    pub fn add<G: Gain<T> + 'a>(&mut self, gain: G) {
        self.gains.push(Box::new(gain));
    }

    pub fn add_boxed(&mut self, gain: Box<dyn Gain<T>>) {
        self.gains.push(gain);
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn gains(&self) -> &[Box<dyn Gain<T> + 'a>] {
        &self.gains
    }

    pub fn gains_mut(&mut self) -> &mut [Box<dyn Gain<T> + 'a>] {
        &mut self.gains
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

    fn set_start_idx(&mut self, idx: Option<u16>) {
        self.start_idx = idx;
    }

    fn start_idx(&self) -> Option<u16> {
        self.start_idx
    }

    fn set_finish_idx(&mut self, idx: Option<u16>) {
        self.finish_idx = idx;
    }

    fn finish_idx(&self) -> Option<u16> {
        self.finish_idx
    }
}

impl<'a, T: Transducer> Sendable<T> for GainSTM<'a, T> {
    type H = autd3_driver::NullHeader;
    type B = T::GainSTM;

    fn operation(
        &mut self,
        geometry: &Geometry<T>,
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
        Ok((
            Self::H::default(),
            <Self::B as autd3_driver::operation::GainSTMOp>::new(drives, props, || {
                geometry.transducers().map(|tr| tr.cycle()).collect()
            }),
        ))
    }
}
