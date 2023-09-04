/*
 * File: mod.rs
 * Project: datagram
 * Created Date: 01/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod clear;
mod filter;
mod gain;
mod mod_delay;
mod modulation;
mod silencer;
mod stm;
mod stop;
mod synchronize;

pub use clear::Clear;
pub use filter::{AmpFilter, PhaseFilter};
pub use gain::{Gain, GainAsAny, GainFilter};
pub use mod_delay::ModDelay;
pub use modulation::{Modulation, ModulationProperty};
pub use silencer::Silencer;
pub use stm::{FocusSTM, GainSTM};
pub use stop::Stop;
pub use synchronize::Synchronize;

use std::{marker::PhantomData, time::Duration};

use crate::{error::AUTDInternalError, geometry::*, operation::Operation};

/// Datagram to be sent to devices
pub trait Datagram<T: Transducer> {
    type O1: Operation<T>;
    type O2: Operation<T>;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError>;

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
    type O1 = D::O1;
    type O2 = D::O2;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        self.datagram.operation()
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

impl<T: Transducer, D> Datagram<T> for Box<D>
where
    D: Datagram<T>,
{
    type O1 = D::O1;
    type O2 = D::O2;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        D::operation(*self)
    }

    fn timeout(&self) -> Option<Duration> {
        D::timeout(self)
    }
}

impl<T: Transducer, D1, D2> Datagram<T> for (D1, D2)
where
    D1: Datagram<T, O2 = crate::operation::NullOp>,
    D2: Datagram<T, O2 = crate::operation::NullOp>,
{
    type O1 = D1::O1;
    type O2 = D2::O1;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let (o1, _) = self.0.operation()?;
        let (o2, _) = self.1.operation()?;
        Ok((o1, o2))
    }
}
