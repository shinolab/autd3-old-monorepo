/*
 * File: interface.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use crate::geometry::{Geometry, Transducer};

pub struct Empty;
pub struct Filled;

pub trait Sendable<T: Transducer> {
    type H;
    type B;
    type O: autd3_driver::Operation;

    fn operation(&mut self, geometry: &Geometry<T>) -> Result<Self::O>;
}

pub trait DatagramHeader {
    type O: autd3_driver::Operation;
    fn operation(&mut self) -> Result<Self::O>;
}

pub trait DatagramBody<T: Transducer> {
    type O: autd3_driver::Operation;
    fn operation(&mut self, geometry: &Geometry<T>) -> Result<Self::O>;
}

#[derive(Default)]
pub struct NullHeader {}

impl NullHeader {
    pub fn new() -> Self {
        Self {}
    }
}

impl DatagramHeader for NullHeader {
    type O = autd3_driver::NullHeader;

    fn operation(&mut self) -> Result<Self::O> {
        Ok(Default::default())
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
    type O = autd3_driver::NullBody;

    fn operation(&mut self, _: &Geometry<T>) -> Result<Self::O> {
        Ok(Default::default())
    }
}
