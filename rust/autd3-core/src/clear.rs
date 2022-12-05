/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use crate::{
    datagram::{DatagramHeader, Empty, Filled, Sendable},
    geometry::{Geometry, Transducer},
};

pub struct Clear {
    sent: bool,
}

impl DatagramHeader for Clear {
    fn init(&mut self) -> Result<()> {
        self.sent = false;
        Ok(())
    }

    fn pack(&mut self, _: u8, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        self.sent = true;
        autd3_driver::clear(tx);
        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}

impl<T: Transducer> Sendable<T> for Clear {
    type H = Empty;
    type B = Filled;

    fn init(&mut self) -> Result<()> {
        DatagramHeader::init(self)
    }

    fn pack(
        &mut self,
        msg_id: u8,
        _geometry: &Geometry<T>,
        tx: &mut autd3_driver::TxDatagram,
    ) -> Result<()> {
        DatagramHeader::pack(self, msg_id, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramHeader::is_finished(self)
    }
}
