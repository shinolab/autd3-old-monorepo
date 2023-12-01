/*
 * File: mod.rs
 * Project: datagram
 * Created Date: 29/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod clear;
mod debug;
mod gain;
mod mod_delay;
mod modulation;
mod silencer;
mod stm;
mod stop;
mod synchronize;
mod update_flag;
mod with_timeout;

pub use clear::Clear;
pub use debug::ConfigureDebugOutputIdx;
pub use gain::{Gain, GainAsAny, GainFilter};
pub use mod_delay::ConfigureModDelay;
pub use modulation::{Modulation, ModulationProperty};
pub use silencer::Silencer;
pub use stm::{FocusSTM, GainSTM, STMProps};
pub use stop::Stop;
pub use synchronize::Synchronize;
pub use update_flag::UpdateFlags;
pub use with_timeout::{DatagramT, DatagramWithTimeout};

use std::time::Duration;

use crate::{error::AUTDInternalError, operation::Operation};

/// Datagram to be sent to devices
pub trait Datagram {
    type O1: Operation;
    type O2: Operation;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration> {
        None
    }
}

impl<D1, D2> Datagram for (D1, D2)
where
    D1: Datagram<O2 = crate::operation::NullOp>,
    D2: Datagram<O2 = crate::operation::NullOp>,
{
    type O1 = D1::O1;
    type O2 = D2::O1;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let (o1, _) = self.0.operation()?;
        let (o2, _) = self.1.operation()?;
        Ok((o1, o2))
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{ClearOp, ConfigureModDelayOp, NullOp};

    use super::*;

    struct TestDatagram1 {
        pub err: bool,
    }
    impl Datagram for TestDatagram1 {
        type O1 = ClearOp;
        type O2 = NullOp;

        fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
            if self.err {
                Err(AUTDInternalError::NotSupported("Err1".to_owned()))
            } else {
                Ok((Self::O1::default(), Self::O2::default()))
            }
        }
    }

    struct TestDatagram2 {
        pub err: bool,
    }
    impl Datagram for TestDatagram2 {
        type O1 = ConfigureModDelayOp;
        type O2 = NullOp;

        fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
            if self.err {
                Err(AUTDInternalError::NotSupported("Err2".to_owned()))
            } else {
                Ok((Self::O1::default(), Self::O2::default()))
            }
        }
    }

    #[test]
    fn test_datagram_tuple() {
        let d = (TestDatagram1 { err: false }, TestDatagram2 { err: false });
        let _: (ClearOp, ConfigureModDelayOp) =
            <(TestDatagram1, TestDatagram2) as Datagram>::operation(d).unwrap();
    }

    #[test]
    fn test_datagram_tuple_err() {
        let d1 = (TestDatagram1 { err: true }, TestDatagram2 { err: false });
        let r = <(TestDatagram1, TestDatagram2) as Datagram>::operation(d1);
        assert!(r.is_err());

        let d2 = (TestDatagram1 { err: false }, TestDatagram2 { err: true });
        let r = <(TestDatagram1, TestDatagram2) as Datagram>::operation(d2);
        assert!(r.is_err());
    }
}
