/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;
use autd3_driver::Operation;

use crate::{
    datagram::{DatagramHeader, Empty, Filled, Sendable},
    geometry::{Geometry, Transducer},
};

#[derive(Default)]
pub struct Clear {
    op: autd3_driver::Clear,
}

impl Clear {
    pub fn new() -> Self {
        Self {
            op: Default::default(),
        }
    }
}

impl DatagramHeader for Clear {
    fn init(&mut self) -> Result<()> {
        self.op.init();
        Ok(())
    }

    fn pack(&mut self, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<T: Transducer> Sendable<T> for Clear {
    type H = Filled;
    type B = Empty;

    fn init(&mut self, _geometry: &Geometry<T>) -> Result<()> {
        DatagramHeader::init(self)
    }

    fn pack(&mut self, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        DatagramHeader::pack(self, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramHeader::is_finished(self)
    }
}
