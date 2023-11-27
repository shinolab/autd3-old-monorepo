/*
 * File: with_timeout.rs
 * Project: datagram
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use super::Datagram;
use crate::error::AUTDInternalError;

/// Datagram with timeout
pub struct DatagramWithTimeout<D: Datagram> {
    datagram: D,
    timeout: Duration,
}

impl<D: Datagram> Datagram for DatagramWithTimeout<D> {
    type O1 = D::O1;
    type O2 = D::O2;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        self.datagram.operation()
    }

    fn timeout(&self) -> Option<Duration> {
        Some(self.timeout)
    }
}

pub trait DatagramT<D: Datagram> {
    /// Set timeout.
    /// This takes precedence over the timeout specified in Link.
    fn with_timeout(self, timeout: Duration) -> DatagramWithTimeout<D>;
}

impl<D: Datagram> DatagramT<D> for D {
    fn with_timeout(self, timeout: Duration) -> DatagramWithTimeout<D> {
        DatagramWithTimeout {
            datagram: self,
            timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{ConfigureModDelayOp, NullOp};

    use super::*;

    struct TestDatagram {}
    impl Datagram for TestDatagram {
        type O1 = ConfigureModDelayOp;
        type O2 = NullOp;

        fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
            Ok((Self::O1::default(), Self::O2::default()))
        }
    }

    #[test]
    fn test_datagram_with_timeout() {
        let d: DatagramWithTimeout<TestDatagram> =
            TestDatagram {}.with_timeout(Duration::from_millis(100));

        let timeout = <DatagramWithTimeout<TestDatagram> as Datagram>::timeout(&d);
        assert!(timeout.is_some());
        assert_eq!(timeout.unwrap(), Duration::from_millis(100));

        let _: (ConfigureModDelayOp, NullOp) =
            <DatagramWithTimeout<TestDatagram> as Datagram>::operation(d).unwrap();
    }
}
