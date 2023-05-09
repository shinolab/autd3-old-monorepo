/*
 * File: sendable.rs
 * Project: src
 * Created Date: 06/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

pub trait Sendable<T: Transducer> {
    type H: autd3_driver::Operation;
    type B: autd3_driver::Operation;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError>;
    fn body_operation(&mut self, geometry: &Geometry<T>) -> Result<Self::B, AUTDInternalError>;

    fn timeout() -> Option<Duration> {
        None
    }
}

impl<T: Transducer, H, B> Sendable<T> for (H, B)
where
    H: Sendable<T, B = autd3_driver::NullBody>,
    B: Sendable<T, H = autd3_driver::NullHeader>,
{
    type H = H::H;
    type B = B::B;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        self.0.header_operation()
    }

    fn body_operation(&mut self, geometry: &Geometry<T>) -> Result<Self::B, AUTDInternalError> {
        self.1.body_operation(geometry)
    }
}

#[derive(Default)]
pub struct NullHeader {}

impl NullHeader {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Sendable<T> for NullHeader {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::NullBody;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(&mut self, _geometry: &Geometry<T>) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::default())
    }
}

#[derive(Default)]
pub struct NullBody {}

impl NullBody {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Sendable<T> for NullBody {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::NullBody;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(&mut self, _geometry: &Geometry<T>) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::default())
    }
}
