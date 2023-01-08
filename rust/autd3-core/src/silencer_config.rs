/*
 * File: silencer_config.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::{DatagramHeader, Empty, Filled, Sendable},
    geometry::{Geometry, Transducer},
};
use anyhow::Result;
use autd3_driver::{Operation, TxDatagram};

pub struct SilencerConfig {
    op: autd3_driver::ConfigSilencer,
}

impl SilencerConfig {
    pub fn new(step: u16, cycle: u16) -> Self {
        let mut op = autd3_driver::ConfigSilencer::default();
        op.step = step;
        op.cycle = cycle;
        SilencerConfig { op }
    }

    pub fn none() -> Self {
        Self::new(0xFFFF, 4096)
    }
}

impl DatagramHeader for SilencerConfig {
    fn init(&mut self) -> Result<()> {
        self.op.init();
        Ok(())
    }

    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<T: Transducer> Sendable<T> for SilencerConfig {
    type H = Filled;
    type B = Empty;

    fn init(&mut self, _geometry: &Geometry<T>) -> Result<()> {
        DatagramHeader::init(self)
    }

    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        DatagramHeader::pack(self, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramHeader::is_finished(self)
    }
}

impl Default for SilencerConfig {
    fn default() -> Self {
        Self::new(10, 4096)
    }
}
