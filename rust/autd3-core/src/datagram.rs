/*
 * File: interface.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::marker::PhantomData;

use autd3_driver::DriverError;

use crate::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

pub struct Empty;
pub struct Filled;

pub trait Sendable<T: Transducer> {
    type H;
    type B;
    type O: autd3_driver::Operation;

    fn operation(&mut self, geometry: &Geometry<T>) -> Result<Self::O, AUTDInternalError>;
}

#[derive(Default)]
pub struct EmptySendable<T: Transducer> {
    _phantom: PhantomData<T>,
}

impl<T: Transducer> EmptySendable<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

pub struct EmptyOperator {}

impl autd3_driver::Operation for EmptyOperator {
    fn init(&mut self) {}
    fn pack(&mut self, _tx: &mut autd3_driver::TxDatagram) -> Result<(), DriverError> {
        Ok(())
    }
    fn is_finished(&self) -> bool {
        true
    }
}

impl<T: Transducer> Sendable<T> for EmptySendable<T> {
    type H = Empty;
    type B = Empty;
    type O = EmptyOperator;

    fn operation(&mut self, _geometry: &Geometry<T>) -> Result<Self::O, AUTDInternalError> {
        Ok(EmptyOperator {})
    }
}

pub trait DatagramHeader {
    type O: autd3_driver::Operation;
    fn operation(&mut self) -> Result<Self::O, AUTDInternalError>;
}

pub trait DatagramBody<T: Transducer> {
    type O: autd3_driver::Operation;
    fn operation(&mut self, geometry: &Geometry<T>) -> Result<Self::O, AUTDInternalError>;
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

    fn operation(&mut self) -> Result<Self::O, AUTDInternalError> {
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

    fn operation(&mut self, _: &Geometry<T>) -> Result<Self::O, AUTDInternalError> {
        Ok(Default::default())
    }
}
