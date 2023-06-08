/*
 * File: gain.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, gain::Gain, geometry::*};

use autd3_driver::{float, GainSTMProps, Mode};

use super::STMProps;

pub struct GainSTM<'a, T: Transducer> {
    gains: Vec<Box<dyn Gain<T> + 'a>>,
    mode: Mode,
    props: STMProps,
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

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn with_props(props: STMProps) -> Self {
        Self {
            gains: Vec::new(),
            mode: Mode::PhaseDutyFull,
            props,
        }
    }

    pub fn with_props_mode(props: STMProps, mode: Mode) -> Self {
        Self {
            gains: Vec::new(),
            mode,
            props,
        }
    }
}

impl<'a, T: Transducer> GainSTM<'a, T> {
    pub fn new(freq: float) -> Self {
        Self {
            gains: vec![],
            mode: Mode::PhaseDutyFull,
            props: STMProps::new(freq),
        }
    }

    pub fn with_sampling_frequency_division(freq_div: u32) -> Self {
        Self {
            gains: vec![],
            mode: Mode::PhaseDutyFull,
            props: STMProps::with_sampling_frequency_division(freq_div),
        }
    }

    pub fn with_sampling_frequency(freq: float) -> Self {
        Self {
            gains: vec![],
            mode: Mode::PhaseDutyFull,
            props: STMProps::with_sampling_frequency(freq),
        }
    }

    pub fn with_start_idx(self, idx: Option<u16>) -> Self {
        Self {
            props: self.props.with_start_idx(idx),
            ..self
        }
    }

    pub fn with_finish_idx(self, idx: Option<u16>) -> Self {
        Self {
            props: self.props.with_finish_idx(idx),
            ..self
        }
    }

    pub fn start_idx(&self) -> Option<u16> {
        self.props.start_idx()
    }

    pub fn finish_idx(&self) -> Option<u16> {
        self.props.finish_idx()
    }

    pub fn size(&self) -> usize {
        self.gains.len()
    }

    pub fn freq(&self) -> float {
        self.props.freq(self.size())
    }

    pub fn sampling_frequency(&self) -> float {
        self.props.sampling_frequency(self.size())
    }

    pub fn sampling_frequency_division(&self) -> u32 {
        self.props.sampling_frequency_division(self.size())
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
            freq_div: self.sampling_frequency_division(),
            finish_idx: self.props.finish_idx,
            start_idx: self.props.start_idx,
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
    use autd3_driver::FPGA_SUB_CLK_FREQ;

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

        let stm = GainSTM::<LegacyTransducer>::with_sampling_frequency_division(512)
            .add_gains_from_iter((0..10).map(|_| Box::new(NullGain {}) as _));
        assert_approx_eq!(stm.freq(), FPGA_SUB_CLK_FREQ as float / 512. / 10.);

        let stm = GainSTM::<LegacyTransducer>::with_sampling_frequency(40e3)
            .add_gains_from_iter((0..10).map(|_| Box::new(NullGain {}) as _));
        assert_approx_eq!(stm.freq(), 40e3 / 10.);
    }

    #[test]
    fn sampling_frequency_division() {
        let stm = GainSTM::<LegacyTransducer>::with_sampling_frequency_division(512);
        assert_eq!(stm.sampling_frequency_division(), 512);

        let stm = GainSTM::<LegacyTransducer>::new(1.0)
            .add_gains_from_iter((0..10).map(|_| Box::new(NullGain {}) as _));
        assert_eq!(
            stm.sampling_frequency_division(),
            (FPGA_SUB_CLK_FREQ as float / 10.) as u32
        );

        let stm = GainSTM::<LegacyTransducer>::with_sampling_frequency(40e3);
        assert_eq!(
            stm.sampling_frequency_division(),
            (FPGA_SUB_CLK_FREQ as float / 40e3) as u32
        );
    }

    #[test]
    fn sampling_frequency() {
        let stm = GainSTM::<LegacyTransducer>::with_sampling_frequency(40e3);
        assert_eq!(stm.sampling_frequency(), 40e3);

        let stm = GainSTM::<LegacyTransducer>::with_sampling_frequency_division(512);
        assert_approx_eq!(stm.sampling_frequency(), FPGA_SUB_CLK_FREQ as float / 512.);

        let stm = GainSTM::<LegacyTransducer>::new(1.0)
            .add_gains_from_iter((0..10).map(|_| Box::new(NullGain {}) as _));
        assert_approx_eq!(stm.sampling_frequency(), 1. * 10.);
    }
}
