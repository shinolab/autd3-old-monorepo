/*
 * File: mod.rs
 * Project: datagram
 * Created Date: 29/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod amplitude;
mod clear;
mod filter;
mod gain;
mod mod_delay;
mod modulation;
mod silencer;
mod stm;
mod stop;
mod synchronize;
mod update_flag;
mod with_timeout;

pub use amplitude::Amplitudes;
pub use clear::Clear;
pub use filter::{ConfigureAmpFilter, ConfigurePhaseFilter};
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

#[cfg(test)]
mod tests {
    use crate::operation::{ConfigureAmpFilterOp, ConfigurePhaseFilterOp, NullOp};

    use super::*;

    struct TestDatagram1 {
        pub err: bool,
    }
    impl<T: Transducer> Datagram<T> for TestDatagram1 {
        type O1 = ConfigureAmpFilterOp;
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
    impl<T: Transducer> Datagram<T> for TestDatagram2 {
        type O1 = ConfigurePhaseFilterOp;
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
        let _: (ConfigureAmpFilterOp, ConfigurePhaseFilterOp) =
            <(TestDatagram1, TestDatagram2) as Datagram<LegacyTransducer>>::operation(d).unwrap();
    }

    #[test]
    fn test_datagram_tuple_err() {
        let d1 = (TestDatagram1 { err: true }, TestDatagram2 { err: false });
        let r = <(TestDatagram1, TestDatagram2) as Datagram<LegacyTransducer>>::operation(d1);
        assert!(r.is_err());

        let d2 = (TestDatagram1 { err: false }, TestDatagram2 { err: true });
        let r = <(TestDatagram1, TestDatagram2) as Datagram<LegacyTransducer>>::operation(d2);
        assert!(r.is_err());
    }
}
