/*
 * File: Datagram.rs
 * Project: src
 * Created Date: 06/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{marker::PhantomData, time::Duration};

use autd3_driver::Operation;

use crate::{error::AUTDInternalError, geometry::*};

/// Datagram to be sent to devices
pub trait Datagram<T: Transducer> {
    /// Header type
    type H: Operation;
    /// Body type
    type B: Operation;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

/// Datagram with timeout
pub struct DatagramWithTimeout<T: Transducer, D: Datagram<T>> {
    datagram: D,
    timeout: Duration,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Transducer, D: Datagram<T>> Datagram<T> for DatagramWithTimeout<T, D> {
    type H = D::H;
    type B = D::B;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        self.datagram.operation(geometry)
    }

    fn timeout(&self) -> Option<Duration> {
        Some(self.timeout)
    }
}

pub trait DatagramT<T: Transducer, D: Datagram<T>> {
    /// Set timeout.
    /// This takes precedence over the timeout specified in Link.
    fn with_timeout(self, timeout: Duration) -> DatagramWithTimeout<T, D>;
}

impl<T: Transducer, D: Datagram<T>> DatagramT<T, D> for D {
    fn with_timeout(self, timeout: Duration) -> DatagramWithTimeout<T, D> {
        DatagramWithTimeout {
            datagram: self,
            timeout,
            phantom: PhantomData,
        }
    }
}

impl<T: Transducer, B> Datagram<T> for Box<B>
where
    B: Datagram<T>,
{
    type H = B::H;
    type B = B::B;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        B::operation(self, geometry)
    }

    fn timeout(&self) -> Option<Duration> {
        B::timeout(self)
    }
}

impl<T: Transducer, B> Datagram<T> for &B
where
    B: Datagram<T>,
{
    type H = B::H;
    type B = B::B;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        B::operation(self, geometry)
    }

    fn timeout(&self) -> Option<Duration> {
        B::timeout(self)
    }
}

impl<T: Transducer, H, B> Datagram<T> for (H, B)
where
    H: Datagram<T, B = autd3_driver::NullBody>,
    B: Datagram<T, H = autd3_driver::NullHeader>,
{
    type H = H::H;
    type B = B::B;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let (h, _) = self.0.operation(geometry)?;
        let (_, b) = self.1.operation(geometry)?;
        Ok((h, b))
    }
}

/// Header to do nothing
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

    fn operation(&self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}

/// Body to do nothing
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

    fn operation(&self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}
