/*
 * File: gain.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    gain::Gain,
    geometry::{Geometry, LegacyTransducer, NormalPhaseTransducer, NormalTransducer, Transducer},
};

use anyhow::{Ok, Result};
use autd3_driver::{GainSTMOp, Mode, Operation, TxDatagram};

use super::STM;

#[derive(Default)]
pub struct GainSTM<'a, T: Transducer> {
    gains: Vec<Box<dyn Gain<T> + 'a>>,
    op: T::GainSTM,
}

impl<'a, T: Transducer> GainSTM<'a, T> {
    pub fn new() -> Self {
        Self {
            gains: vec![],
            op: Default::default(),
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
        self.op.set_sampling_freq_div(freq_div);
    }

    fn sampling_freq_div(&self) -> u32 {
        self.op.sampling_freq_div()
    }

    fn set_start_idx(&mut self, idx: Option<u16>) {
        self.op.set_start_idx(idx)
    }

    fn start_idx(&self) -> Option<u16> {
        self.op.start_idx()
    }

    fn set_finish_idx(&mut self, idx: Option<u16>) {
        self.op.set_finish_idx(idx)
    }

    fn finish_idx(&self) -> Option<u16> {
        self.op.finish_idx()
    }
}

impl<'a> GainSTM<'a, LegacyTransducer> {
    pub fn mode(&self) -> Mode {
        self.op.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.op.mode = mode;
    }
}

impl<'a> GainSTM<'a, NormalTransducer> {
    pub fn mode(&self) -> Mode {
        self.op.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.op.mode = mode;
    }
}

impl<'a> DatagramBody<NormalTransducer> for GainSTM<'a, NormalTransducer> {
    fn init(&mut self, geometry: &Geometry<NormalTransducer>) -> Result<()> {
        self.op.init();
        self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        for gain in &mut self.gains {
            gain.init(geometry)?;
            self.op.drives.push(gain.drives().to_vec());
        }
        Ok(())
    }

    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<'a> DatagramBody<NormalPhaseTransducer> for GainSTM<'a, NormalPhaseTransducer> {
    fn init(&mut self, geometry: &Geometry<NormalPhaseTransducer>) -> Result<()> {
        self.op.init();
        self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        for gain in &mut self.gains {
            gain.init(geometry)?;
            self.op.drives.push(gain.drives().to_vec());
        }
        Ok(())
    }

    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<'a> DatagramBody<LegacyTransducer> for GainSTM<'a, LegacyTransducer> {
    fn init(&mut self, geometry: &Geometry<LegacyTransducer>) -> Result<()> {
        self.op.init();
        for gain in &mut self.gains {
            gain.init(geometry)?;
            self.op.drives.push(gain.drives().to_vec());
        }
        Ok(())
    }

    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<'a, T: Transducer> Sendable<T> for GainSTM<'a, T>
where
    GainSTM<'a, T>: DatagramBody<T>,
{
    type H = Empty;
    type B = Filled;

    fn init(&mut self, geometry: &Geometry<T>) -> Result<()> {
        DatagramBody::<T>::init(self, geometry)
    }

    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        DatagramBody::<T>::pack(self, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<T>::is_finished(self)
    }
}
