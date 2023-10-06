/*
 * File: dynamic_op.rs
 * Project: src
 * Created Date: 06/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    datagram::GainFilter,
    defined::Drive,
    operation::{
        GainOp, GainSTMAdvancedOp, GainSTMAdvancedPhaseOp, GainSTMLegacyOp, GainSTMMode, Operation,
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
        devices: &autd3_driver::geometry::Geometry<DynamicTransducer>,
    ) -> Result<(), autd3_driver::error::AUTDInternalError> {
        self.drives = self.gain.calc(devices, GainFilter::All)?;
        match self.mode {
            TransMode::Legacy | TransMode::AdvancedPhase => {
                self.remains = devices.iter().map(|device| (device.idx(), 1)).collect()
            }
            TransMode::Advanced => {
                self.remains = devices.iter().map(|device| (device.idx(), 2)).collect()
            }
        }
        Ok(())
    }

    fn required_size(&self, device: &autd3_driver::geometry::Device<DynamicTransducer>) -> usize {
        GainOp::<DynamicTransducer, Box<G>>::required_size_impl(device)
    }

    fn pack(
        &mut self,
        device: &autd3_driver::geometry::Device<DynamicTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, autd3_driver::error::AUTDInternalError> {
        match self.mode {
            TransMode::Legacy => GainOp::<DynamicTransducer, Box<G>>::pack_legacy(
                &self.drives,
                &self.remains,
                device,
                tx,
            ),
            TransMode::Advanced => GainOp::<DynamicTransducer, Box<G>>::pack_advanced(
                &self.drives,
                &self.remains,
                device,
                tx,
            ),
            TransMode::AdvancedPhase => GainOp::<DynamicTransducer, Box<G>>::pack_advanced_phase(
                &self.drives,
                &self.remains,
                device,
                tx,
            ),
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
        match self.mode {
            TransMode::Legacy => GainSTMLegacyOp::<DynamicTransducer, Box<G>>::init_impl(
                &self.gains,
                &mut self.drives,
                &mut self.remains,
                &mut self.sent,
                self.gain_stm_mode,
                self.freq_div,
                geometry,
            ),
            TransMode::Advanced => GainSTMAdvancedOp::<DynamicTransducer, Box<G>>::init_impl(
                &self.gains,
                &mut self.drives,
                &mut self.remains,
                &mut self.sent,
                self.gain_stm_mode,
                self.freq_div,
                geometry,
            ),
            TransMode::AdvancedPhase => {
                GainSTMAdvancedPhaseOp::<DynamicTransducer, Box<G>>::init_impl(
                    &self.gains,
                    &mut self.drives,
                    &mut self.remains,
                    &mut self.sent,
                    self.gain_stm_mode,
                    self.freq_div,
                    geometry,
                )
            }
        }
    }

    fn required_size(&self, device: &autd3_driver::geometry::Device<DynamicTransducer>) -> usize {
        match self.mode {
            TransMode::Legacy => {
                GainSTMLegacyOp::<DynamicTransducer, Box<G>>::required_size_impl(&self.sent, device)
            }
            TransMode::Advanced => {
                GainSTMAdvancedOp::<DynamicTransducer, Box<G>>::required_size_impl(
                    &self.sent, device,
                )
            }
            TransMode::AdvancedPhase => {
                GainSTMAdvancedPhaseOp::<DynamicTransducer, Box<G>>::required_size_impl(
                    &self.sent, device,
                )
            }
        }
    }

    fn pack(
        &mut self,
        device: &autd3_driver::geometry::Device<DynamicTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, autd3_driver::error::AUTDInternalError> {
        match self.mode {
            TransMode::Legacy => GainSTMLegacyOp::<DynamicTransducer, Box<G>>::pack_impl(
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
            TransMode::Advanced => GainSTMAdvancedOp::<DynamicTransducer, Box<G>>::pack_impl(
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
            TransMode::AdvancedPhase => {
                GainSTMAdvancedPhaseOp::<DynamicTransducer, Box<G>>::pack_impl(
                    &self.drives,
                    &self.remains,
                    &mut self.sent,
                    self.gain_stm_mode,
                    self.freq_div,
                    self.start_idx,
                    self.finish_idx,
                    device,
                    tx,
                )
            }
        }
    }

    fn commit(&mut self, device: &autd3_driver::geometry::Device<DynamicTransducer>) {
        match self.mode {
            TransMode::Legacy => GainSTMLegacyOp::<DynamicTransducer, Box<G>>::commit_impl(
                &self.gains,
                &mut self.remains,
                &self.sent,
                self.gain_stm_mode,
                device,
            ),
            TransMode::Advanced => GainSTMAdvancedOp::<DynamicTransducer, Box<G>>::commit_impl(
                &self.gains,
                &mut self.remains,
                &self.sent,
                self.gain_stm_mode,
                device,
            ),
            TransMode::AdvancedPhase => {
                GainSTMAdvancedPhaseOp::<DynamicTransducer, Box<G>>::commit_impl(
                    &self.gains,
                    &mut self.remains,
                    &self.sent,
                    self.gain_stm_mode,
                    device,
                )
            }
        };
    }

    fn remains(&self, device: &autd3_driver::geometry::Device<DynamicTransducer>) -> usize {
        self.remains[&device.idx()]
    }
}
