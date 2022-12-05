/*
 * File: delay.rs
 * Project: src
 * Created Date: 01/06/2022
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
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{Geometry, Transducer},
};

pub struct ModDelay {
    sent: bool,
}

impl<T: Transducer> DatagramBody<T> for ModDelay {
    fn init(&mut self) -> Result<()> {
        self.sent = false;
        Ok(())
    }

    fn pack(&mut self, geometry: &Geometry<T>, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        autd3_driver::null_body(tx);
        if DatagramBody::<T>::is_finished(self) {
            return Ok(());
        }

        let delays: Vec<u16> = geometry.transducers().map(|tr| tr.mod_delay()).collect();

        self.sent = true;
        autd3_driver::mod_delay(&delays, tx)?;
        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}

impl<T: Transducer> Sendable<T> for ModDelay {
    type H = Empty;
    type B = Filled;

    fn init(&mut self) -> Result<()> {
        DatagramBody::<T>::init(self)
    }

    fn pack(
        &mut self,
        _msg_id: u8,
        geometry: &Geometry<T>,
        tx: &mut autd3_driver::TxDatagram,
    ) -> Result<()> {
        DatagramBody::<T>::pack(self, geometry, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<T>::is_finished(self)
    }
}
