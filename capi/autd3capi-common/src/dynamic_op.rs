/*
 * File: dynamic_op.rs
 * Project: src
 * Created Date: 06/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    datagram::GainFilter,
    defined::Drive,
    error::AUTDInternalError,
    fpga::{SAMPLING_FREQ_DIV_MAX, SAMPLING_FREQ_DIV_MIN},
    operation::{
        gain::{
            advanced::GainOpAdvanced, advanced_phase::GainOpAdvancedPhase, legacy::GainOpLegacy,
            GainOpDelegate,
        },
        stm::gain::{
            advanced::GainSTMOpAdvanced, advanced_phase::GainSTMOpAdvancedPhase,
            legacy::GainSTMOpLegacy, GainSTMOpDelegate,
        },
        GainSTMMode, Operation,
    },
};

use crate::{dynamic_transducer::TransMode, DynamicTransducer, G};

pub struct DynamicGainOp {
    mode: TransMode,
    gain: Box<G>,
    drives: HashMap<usize, Vec<Drive>>,
    remains: HashMap<usize, usize>,
}

impl DynamicGainOp {
    pub fn new(gain: Box<G>, mode: TransMode) -> Self {
        Self {
            mode,
            gain,
            drives: Default::default(),
            remains: Default::default(),
        }
    }
}

impl Operation<DynamicTransducer> for DynamicGainOp {
    fn init(
        &mut self,
        geometry: &autd3_driver::geometry::Geometry<DynamicTransducer>,
    ) -> Result<(), autd3_driver::error::AUTDInternalError> {
        self.drives = self.gain.calc(geometry, GainFilter::All)?;
        self.remains = match self.mode {
            TransMode::Legacy => GainOpLegacy::init(geometry),
            TransMode::Advanced => GainOpAdvanced::init(geometry),
            TransMode::AdvancedPhase => GainOpAdvancedPhase::init(geometry),
        }?;
        Ok(())
    }

    fn required_size(&self, device: &autd3_driver::geometry::Device<DynamicTransducer>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<u16>()
    }

    fn pack(
        &mut self,
        device: &autd3_driver::geometry::Device<DynamicTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, autd3_driver::error::AUTDInternalError> {
        match self.mode {
            TransMode::Legacy => GainOpLegacy::pack(&self.drives, &self.remains, device, tx),
            TransMode::Advanced => GainOpAdvanced::pack(&self.drives, &self.remains, device, tx),
            TransMode::AdvancedPhase => {
                GainOpAdvancedPhase::pack(&self.drives, &self.remains, device, tx)
            }
        }
    }

    fn commit(&mut self, device: &autd3_driver::geometry::Device<DynamicTransducer>) {
        self.remains
            .insert(device.idx(), self.remains[&device.idx()] - 1);
    }

    fn remains(&self, device: &autd3_driver::geometry::Device<DynamicTransducer>) -> usize {
        self.remains[&device.idx()]
    }
}

pub struct DynamicGainSTMOp {
    mode: TransMode,
    gains: Vec<Box<G>>,
    drives: Vec<HashMap<usize, Vec<Drive>>>,
    remains: HashMap<usize, usize>,
    sent: HashMap<usize, usize>,
    gain_stm_mode: GainSTMMode,
    freq_div: u32,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
}

impl DynamicGainSTMOp {
    pub fn new(
        mode: TransMode,
        gains: Vec<Box<G>>,
        gain_stm_mode: GainSTMMode,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
    ) -> Self {
        Self {
            mode,
            gains,
            drives: Default::default(),
            remains: Default::default(),
            sent: Default::default(),
            gain_stm_mode,
            freq_div,
            start_idx,
            finish_idx,
        }
    }
}

impl Operation<DynamicTransducer> for DynamicGainSTMOp {
    fn init(
        &mut self,
        geometry: &autd3_driver::geometry::Geometry<DynamicTransducer>,
    ) -> Result<(), autd3_driver::error::AUTDInternalError> {
        if !(SAMPLING_FREQ_DIV_MIN..=SAMPLING_FREQ_DIV_MAX).contains(&self.freq_div) {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(self.freq_div));
        }

        self.drives = self
            .gains
            .iter()
            .map(|g| g.calc(geometry, GainFilter::All))
            .collect::<Result<_, _>>()?;

        self.remains = match self.mode {
            TransMode::Legacy => {
                GainSTMOpLegacy::init(self.drives.len(), self.gain_stm_mode, geometry)
            }
            TransMode::Advanced => {
                GainSTMOpAdvanced::init(self.drives.len(), self.gain_stm_mode, geometry)
            }
            TransMode::AdvancedPhase => {
                GainSTMOpAdvancedPhase::init(self.drives.len(), self.gain_stm_mode, geometry)
            }
        }?;

        self.sent = geometry.devices().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    fn required_size(&self, device: &autd3_driver::geometry::Device<DynamicTransducer>) -> usize {
        match self.mode {
            TransMode::Legacy => GainSTMOpLegacy::required_size(&self.sent, device),
            TransMode::Advanced => GainSTMOpAdvanced::required_size(&self.sent, device),
            TransMode::AdvancedPhase => GainSTMOpAdvancedPhase::required_size(&self.sent, device),
        }
    }

    fn pack(
        &mut self,
        device: &autd3_driver::geometry::Device<DynamicTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, autd3_driver::error::AUTDInternalError> {
        assert!(self.remains[&device.idx()] > 0);
        match self.mode {
            TransMode::Legacy => GainSTMOpLegacy::pack(
                &self.drives,
                &self.remains,
                &mut self.sent,
                self.gain_stm_mode,
                self.freq_div,
                self.start_idx,
                self.finish_idx,
                device,
                tx,
            ),
            TransMode::Advanced => GainSTMOpAdvanced::pack(
                &self.drives,
                &self.remains,
                &mut self.sent,
                self.gain_stm_mode,
                self.freq_div,
                self.start_idx,
                self.finish_idx,
                device,
                tx,
            ),
            TransMode::AdvancedPhase => GainSTMOpAdvancedPhase::pack(
                &self.drives,
                &self.remains,
                &mut self.sent,
                self.gain_stm_mode,
                self.freq_div,
                self.start_idx,
                self.finish_idx,
                device,
                tx,
            ),
        }
    }

    fn commit(&mut self, device: &autd3_driver::geometry::Device<DynamicTransducer>) {
        match self.mode {
            TransMode::Legacy => GainSTMOpLegacy::commit(
                &self.drives,
                &mut self.remains,
                &self.sent,
                self.gain_stm_mode,
                device,
            ),
            TransMode::Advanced => GainSTMOpAdvanced::commit(
                &self.drives,
                &mut self.remains,
                &self.sent,
                self.gain_stm_mode,
                device,
            ),
            TransMode::AdvancedPhase => GainSTMOpAdvancedPhase::commit(
                &self.drives,
                &mut self.remains,
                &self.sent,
                self.gain_stm_mode,
                device,
            ),
        };
    }

    fn remains(&self, device: &autd3_driver::geometry::Device<DynamicTransducer>) -> usize {
        self.remains[&device.idx()]
    }
}
