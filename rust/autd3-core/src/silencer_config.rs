/*
 * File: silencer_config.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    geometry::{Geometry, Transducer},
    datagram::{DatagramHeader, Empty, Filled, Sendable},
};
use anyhow::Result;
use autd3_driver::TxDatagram;

pub struct SilencerConfig {
    pub(crate) step: u16,
    pub(crate) cycle: u16,
    sent: bool,
}

impl SilencerConfig {
    pub fn new(step: u16, cycle: u16) -> Self {
        SilencerConfig {
            step,
            cycle,
            sent: false,
        }
    }

    pub fn none() -> Self {
        Self::new(0xFFFF, 4096)
    }
}

impl DatagramHeader for SilencerConfig {
    fn init(&mut self) -> Result<()> {
        self.sent = false;
        Ok(())
    }

    fn pack(&mut self, msg_id: u8, tx: &mut TxDatagram) -> Result<()> {
        if self.sent {
            autd3_driver::null_header(msg_id, tx);
            Ok(())
        } else {
            self.sent = true;
            autd3_driver::config_silencer(msg_id, self.cycle, self.step, tx)
        }
    }

    fn is_finished(&self) -> bool {
        true
    }
}

impl<T: Transducer> Sendable<T> for SilencerConfig {
    type H = Filled;
    type B = Empty;

    fn init(&mut self) -> Result<()> {
        DatagramHeader::init(self)
    }

    fn pack(&mut self, msg_id: u8, _geometry: &Geometry<T>, tx: &mut TxDatagram) -> Result<()> {
        DatagramHeader::pack(self, msg_id, tx)
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
