/*
 * File: Datagram.rs
 * Project: src
 * Created Date: 06/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_driver::Operation;

use crate::{error::AUTDInternalError, geometry::*};

pub trait Datagram<T: Transducer> {
    type H: Operation;
    type B: Operation;

    fn operation(
        &mut self,
        geometry: &Geometry<T>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl<T: Transducer, B> Datagram<T> for Box<B>
where
    B: Datagram<T>,
{
    type H = B::H;
    type B = B::B;

    fn operation(
        &mut self,
        geometry: &Geometry<T>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        B::operation(self, geometry)
    }
}

impl<T: Transducer, H, B> Datagram<T> for (H, B)
where
    H: Datagram<T, B = autd3_driver::NullBody>,
    B: Datagram<T, H = autd3_driver::NullHeader>,
{
    type H = H::H;
    type B = B::B;

    fn operation(
        &mut self,
        geometry: &Geometry<T>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let (h, _) = self.0.operation(geometry)?;
        let (_, b) = self.1.operation(geometry)?;
        Ok((h, b))
    }
}

impl<T: Transducer, H, B> Datagram<T> for (H, B, std::time::Duration)
where
    H: Datagram<T, B = autd3_driver::NullBody>,
    B: Datagram<T, H = autd3_driver::NullHeader>,
{
    type H = H::H;
    type B = B::B;

    fn operation(
        &mut self,
        geometry: &Geometry<T>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let (h, _) = self.0.operation(geometry)?;
        let (_, b) = self.1.operation(geometry)?;
        Ok((h, b))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(self.2)
    }
}

#[derive(Default)]
pub struct NullHeader {}

impl NullHeader {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for NullHeader {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::NullBody;

    fn operation(&mut self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}

#[derive(Default)]
pub struct NullBody {}

impl NullBody {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for NullBody {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::NullBody;

    fn operation(&mut self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}
