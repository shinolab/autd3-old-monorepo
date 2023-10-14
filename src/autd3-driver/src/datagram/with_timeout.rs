/*
 * File: with_timeout.rs
 * Project: datagram
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{marker::PhantomData, time::Duration};

use super::Datagram;
use crate::{error::AUTDInternalError, geometry::*};

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

#[cfg(test)]
mod tests {
    use crate::operation::{ConfigureAmpFilterOp, NullOp};

    use super::*;

    struct TestDatagram {}
    impl<T: Transducer> Datagram<T> for TestDatagram {
        type O1 = ConfigureAmpFilterOp;
        type O2 = NullOp;

        fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
            Ok((Self::O1::default(), Self::O2::default()))
        }
    }

    #[test]
    fn test_datagram_with_timeout() {
        let d: DatagramWithTimeout<LegacyTransducer, TestDatagram> =
            TestDatagram {}.with_timeout(Duration::from_millis(100));

        let timeout = <DatagramWithTimeout<LegacyTransducer, TestDatagram> as Datagram<
            LegacyTransducer,
        >>::timeout(&d);
        assert!(timeout.is_some());
        assert_eq!(timeout.unwrap(), Duration::from_millis(100));

        let _:(ConfigureAmpFilterOp, NullOp) = <DatagramWithTimeout<LegacyTransducer, TestDatagram> as Datagram<
            LegacyTransducer,
        >>::operation(d)
        .unwrap();
    }
}
