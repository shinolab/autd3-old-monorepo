/*
 * File: interface.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{Operation, TxDatagram};

use crate::geometry::{Geometry, Transducer};
use anyhow::Result;

pub struct Empty;
pub struct Filled;

pub trait Sendable<T: Transducer> {
    type H;
    type B;
    fn init(&mut self, geometry: &Geometry<T>) -> Result<()>;
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()>;
    fn is_finished(&self) -> bool;
}

pub trait DatagramHeader {
    fn init(&mut self) -> Result<()>;
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()>;
    fn is_finished(&self) -> bool;
}

pub trait DatagramBody<T: Transducer> {
    fn init(&mut self, geometry: &Geometry<T>) -> Result<()>;
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()>;
    fn is_finished(&self) -> bool;
}

#[derive(Default)]
pub struct NullHeader {
    op: autd3_driver::NullHeader,
}

impl NullHeader {
    pub fn new() -> Self {
        Self {
            op: Default::default(),
        }
    }
}

impl DatagramHeader for NullHeader {
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

#[derive(Default)]
pub struct NullBody {
    op: autd3_driver::NullBody,
}

impl NullBody {
    pub fn new() -> Self {
        Self {
            op: Default::default(),
        }
    }
}

impl<T: Transducer> DatagramBody<T> for NullBody {
    fn init(&mut self, _geometry: &Geometry<T>) -> Result<()> {
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
