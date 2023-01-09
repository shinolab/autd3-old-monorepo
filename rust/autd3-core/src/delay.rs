/*
 * File: delay.rs
 * Project: src
 * Created Date: 01/06/2022
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
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{Geometry, Transducer},
};

pub struct ModDelay {
    op: autd3_driver::ModDelay,
}

impl<T: Transducer> DatagramBody<T> for ModDelay {
    fn init(&mut self, geometry: &Geometry<T>) -> Result<()> {
        self.op.init();
        self.op.delays = geometry.transducers().map(|tr| tr.mod_delay()).collect();
        Ok(())
    }

    fn pack(&mut self, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<T: Transducer> Sendable<T> for ModDelay {
    type H = Empty;
    type B = Filled;

    fn init(&mut self, geometry: &Geometry<T>) -> Result<()> {
        DatagramBody::<T>::init(self, geometry)
    }

    fn pack(&mut self, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        DatagramBody::<T>::pack(self, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<T>::is_finished(self)
    }
}
