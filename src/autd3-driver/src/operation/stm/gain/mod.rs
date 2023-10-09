/*
 * File: mod.rs
 * Project: gain
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod advanced;
pub mod advanced_phase;
mod gain_stm_control_flags;
mod gain_stm_mode;
pub mod legacy;

use std::collections::HashMap;

use crate::{
    derive::prelude::{
        AUTDInternalError, Drive, Gain, GainFilter, Geometry, Operation, Transducer,
    },
    fpga::{SAMPLING_FREQ_DIV_MAX, SAMPLING_FREQ_DIV_MIN},
    geometry::Device,
};

pub use gain_stm_mode::GainSTMMode;

pub struct GainSTMOp<T: Transducer, G: Gain<T>> {
    gains: Vec<G>,
    drives: Vec<HashMap<usize, Vec<Drive>>>,
    remains: HashMap<usize, usize>,
    sent: HashMap<usize, usize>,
    mode: GainSTMMode,
    freq_div: u32,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
    phantom: std::marker::PhantomData<T>,
}

pub trait GainSTMOpDelegate<T: Transducer> {
    fn init(
        n: usize,
        mode: GainSTMMode,
        geometry: &Geometry<T>,
    ) -> Result<HashMap<usize, usize>, AUTDInternalError>;
    #[allow(clippy::too_many_arguments)]
    fn pack(
        drives: &[HashMap<usize, Vec<Drive>>],
        remains: &HashMap<usize, usize>,
        sent_map: &mut HashMap<usize, usize>,
        mode: GainSTMMode,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError>;
    fn commit(
        drives: &[HashMap<usize, Vec<Drive>>],
        remains: &mut HashMap<usize, usize>,
        sent: &HashMap<usize, usize>,
        mode: GainSTMMode,
        device: &Device<T>,
    );
    fn required_size(sent: &HashMap<usize, usize>, device: &Device<T>) -> usize;
}

impl<T: Transducer, G: Gain<T>> GainSTMOp<T, G> {
    pub fn new(
        gains: Vec<G>,
        mode: GainSTMMode,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
    ) -> Self {
        Self {
            gains,
            drives: Default::default(),
            remains: Default::default(),
            sent: Default::default(),
            mode,
            freq_div,
            start_idx,
            finish_idx,
            phantom: Default::default(),
        }
    }
}

impl<T: Transducer, G: Gain<T>> Operation<T> for GainSTMOp<T, G> {
    fn init(
        &mut self,
        geometry: &crate::derive::prelude::Geometry<T>,
    ) -> Result<(), crate::derive::prelude::AUTDInternalError> {
        if !(SAMPLING_FREQ_DIV_MIN..=SAMPLING_FREQ_DIV_MAX).contains(&self.freq_div) {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(self.freq_div));
        }

        self.drives = self
            .gains
            .iter()
            .map(|g| g.calc(geometry, GainFilter::All))
            .collect::<Result<_, _>>()?;

        self.remains = T::GainSTMOp::init(self.drives.len(), self.mode, geometry)?;

        self.sent = geometry.devices().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        T::GainSTMOp::required_size(&self.sent, device)
    }

    fn pack(
        &mut self,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, crate::derive::prelude::AUTDInternalError> {
        assert!(self.remains[&device.idx()] > 0);
        T::GainSTMOp::pack(
            &self.drives,
            &self.remains,
            &mut self.sent,
            self.mode,
            self.freq_div,
            self.start_idx,
            self.finish_idx,
            device,
            tx,
        )
    }

    fn commit(&mut self, device: &Device<T>) {
        T::GainSTMOp::commit(
            &self.drives,
            &mut self.remains,
            &self.sent,
            self.mode,
            device,
        )
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }
}
