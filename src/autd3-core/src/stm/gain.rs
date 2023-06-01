/*
 * File: gain.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, gain::Gain, geometry::*};

use autd3_driver::{float, GainSTMProps, Mode, FPGA_SUB_CLK_FREQ};

use super::STM;

#[derive(Default)]
pub struct GainSTM<'a, T: Transducer> {
    gains: Vec<Box<dyn Gain<T> + 'a>>,
    mode: Mode,
    freq: float,
    freq_div: Option<u32>,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
}

impl<'a, T: Transducer> GainSTM<'a, T> {
    pub fn with_mode(self, mode: Mode) -> Self {
        Self { mode, ..self }
    }
}

impl<'a, T: Transducer> GainSTM<'a, T> {
    pub fn add_gain<G: Gain<T> + 'a>(mut self, gain: G) -> Self {
        self.gains.push(Box::new(gain));
        self
    }

    pub fn add_gain_boxed(mut self, gain: Box<dyn Gain<T>>) -> Self {
        self.gains.push(gain);
        self
    }

    pub fn add_gains_from_iter<I: IntoIterator<Item = Box<dyn Gain<T> + 'a>>>(
        mut self,
        iter: I,
    ) -> Self {
        self.gains.extend(iter);
        self
    }

    pub fn gains(&self) -> &[Box<dyn Gain<T> + 'a>] {
        &self.gains
    }

    pub fn gains_mut(&mut self) -> &mut [Box<dyn Gain<T> + 'a>] {
        &mut self.gains
    }
}

impl<'a, T: Transducer> STM for GainSTM<'a, T> {
    fn new(freq: float) -> Self {
        Self {
            gains: vec![],
            mode: Mode::PhaseDutyFull,
            freq,
            freq_div: None,
            start_idx: None,
            finish_idx: None,
        }
    }

    fn with_sampling_freq_div(freq_div: u32) -> Self {
        Self {
            gains: vec![],
            mode: Mode::PhaseDutyFull,
            freq_div: Some(freq_div),
            freq: 0.,
            start_idx: None,
            finish_idx: None,
        }
    }

    fn with_sampling_freq(freq: float) -> Self {
        Self {
            gains: vec![],
            mode: Mode::PhaseDutyFull,
            freq_div: Some((FPGA_SUB_CLK_FREQ as float / freq) as u32),
            freq: 0.,
            start_idx: None,
            finish_idx: None,
        }
    }

    fn with_start_idx(self, idx: Option<u16>) -> Self {
        Self {
            start_idx: idx,
            ..self
        }
    }

    fn with_finish_idx(self, idx: Option<u16>) -> Self {
        Self {
            finish_idx: idx,
            ..self
        }
    }

    fn size(&self) -> usize {
        self.gains.len()
    }

    fn start_idx(&self) -> Option<u16> {
        self.start_idx
    }

    fn finish_idx(&self) -> Option<u16> {
        self.finish_idx
    }

    fn freq(&self) -> f64 {
        self.freq_div.map_or(self.freq, |div| {
            FPGA_SUB_CLK_FREQ as float / div as float / self.size() as float
        })
    }

    fn sampling_freq(&self) -> f64 {
        self.freq_div
            .map_or((self.freq * self.size() as float) as _, |div| {
                FPGA_SUB_CLK_FREQ as float / div as float
            })
    }

    fn sampling_freq_div(&self) -> u32 {
        self.freq_div
            .unwrap_or((FPGA_SUB_CLK_FREQ as float / (self.freq * self.size() as float)) as _)
    }
}

impl<'a, T: Transducer> Datagram<T> for GainSTM<'a, T> {
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
            freq_div: self.sampling_freq_div(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    struct NullGain {}

    impl<T: Transducer> Gain<T> for NullGain {
        fn calc(&mut self, _: &Geometry<T>) -> Result<Vec<autd3_driver::Drive>, AUTDInternalError> {
            unimplemented!()
        }
    }

    #[test]
    fn freq() {
        let stm = GainSTM::<LegacyTransducer>::new(1.0);
        assert_eq!(stm.freq(), 1.0);

        let stm = GainSTM::<LegacyTransducer>::with_sampling_freq_div(512)
            .add_gains_from_iter((0..10).map(|_| Box::new(NullGain {}) as _));
        assert_approx_eq!(stm.freq(), FPGA_SUB_CLK_FREQ as float / 512. / 10.);

        let stm = GainSTM::<LegacyTransducer>::with_sampling_freq(40e3)
            .add_gains_from_iter((0..10).map(|_| Box::new(NullGain {}) as _));
        assert_approx_eq!(stm.freq(), 40e3 / 10.);
    }

    #[test]
    fn sampling_freq_div() {
        let stm = GainSTM::<LegacyTransducer>::with_sampling_freq_div(512);
        assert_eq!(stm.sampling_freq_div(), 512);

        let stm = GainSTM::<LegacyTransducer>::new(1.0)
            .add_gains_from_iter((0..10).map(|_| Box::new(NullGain {}) as _));
        assert_eq!(
            stm.sampling_freq_div(),
            (FPGA_SUB_CLK_FREQ as float / 10.) as u32
        );

        let stm = GainSTM::<LegacyTransducer>::with_sampling_freq(40e3);
        assert_eq!(
            stm.sampling_freq_div(),
            (FPGA_SUB_CLK_FREQ as float / 40e3) as u32
        );
    }

    #[test]
    fn sampling_freq() {
        let stm = GainSTM::<LegacyTransducer>::with_sampling_freq(40e3);
        assert_eq!(stm.sampling_freq(), 40e3);

        let stm = GainSTM::<LegacyTransducer>::with_sampling_freq_div(512);
        assert_approx_eq!(stm.sampling_freq(), FPGA_SUB_CLK_FREQ as float / 512.);

        let stm = GainSTM::<LegacyTransducer>::new(1.0)
            .add_gains_from_iter((0..10).map(|_| Box::new(NullGain {}) as _));
        assert_approx_eq!(stm.sampling_freq(), 1. * 10.);
    }
}
