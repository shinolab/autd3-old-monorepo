/*
 * File: synchronize.rs
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
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{Geometry, LegacyTransducer, NormalPhaseTransducer, NormalTransducer, Transducer},
};

#[derive(Default)]
pub struct Synchronize<T: Transducer> {
    op: T::Sync,
}

impl DatagramBody<LegacyTransducer> for Synchronize<LegacyTransducer> {
    fn init(&mut self, _geometry: &Geometry<LegacyTransducer>) -> Result<()> {
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

impl DatagramBody<NormalTransducer> for Synchronize<NormalTransducer> {
    fn init(&mut self, geometry: &Geometry<NormalTransducer>) -> Result<()> {
        self.op.init();
        self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(())
    }

    fn pack(&mut self, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl DatagramBody<NormalPhaseTransducer> for Synchronize<NormalPhaseTransducer> {
    fn init(&mut self, geometry: &Geometry<NormalPhaseTransducer>) -> Result<()> {
        self.op.init();
        self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(())
    }

    fn pack(&mut self, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<T: Transducer> Sendable<T> for Synchronize<T>
where
    Synchronize<T>: DatagramBody<T>,
{
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
