/*
 * File: interface.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::TxDatagram;

use crate::geometry::{Geometry, Transducer};
use anyhow::Result;

pub struct Empty;
pub struct Filled;

pub trait Sendable<T: Transducer> {
    type H;
    type B;
    fn init(&mut self) -> Result<()>;
    fn pack(&mut self, msg_id: u8, geometry: &Geometry<T>, tx: &mut TxDatagram) -> Result<()>;
    fn is_finished(&self) -> bool;
}

pub trait DatagramHeader {
    fn init(&mut self) -> Result<()>;
    fn pack(&mut self, msg_id: u8, tx: &mut TxDatagram) -> Result<()>;
    fn is_finished(&self) -> bool;
}

pub trait DatagramBody<T: Transducer> {
    fn init(&mut self) -> Result<()>;
    fn pack(&mut self, geometry: &Geometry<T>, tx: &mut TxDatagram) -> Result<()>;
    fn is_finished(&self) -> bool;
}

#[derive(Default)]
pub struct NullHeader {}

impl NullHeader {
    pub fn new() -> Self {
        Self {}
    }
}

impl DatagramHeader for NullHeader {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn pack(&mut self, msg_id: u8, tx: &mut TxDatagram) -> Result<()> {
        autd3_driver::null_header(msg_id, tx);
        Ok(())
    }

    fn is_finished(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct NullBody {}

impl NullBody {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> DatagramBody<T> for NullBody {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn pack(&mut self, _geometry: &Geometry<T>, tx: &mut TxDatagram) -> Result<()> {
        autd3_driver::null_body(tx);
        Ok(())
    }

    fn is_finished(&self) -> bool {
        true
    }
}
