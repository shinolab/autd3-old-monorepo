/*
 * File: stop.rs
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

use crate::{
    datagram::*,
    error::AUTDInternalError,
    geometry::*,
    operation::{ConfigSilencerOp, StopOp},
};

/// Datagram to stop output
#[derive(Default)]
pub struct Stop {}

impl Stop {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for Stop {
    type O1 = ConfigSilencerOp;
    type O2 = StopOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(10), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_timeout() {
        let stop = Stop::new();
        let timeout = <Stop as Datagram<LegacyTransducer>>::timeout(&stop);
        assert!(timeout.is_none());
    }

    #[test]
    fn test_stop_operation() {
        let stop = Stop::default();
        let r = <Stop as Datagram<LegacyTransducer>>::operation(stop);
        assert!(r.is_ok());
        let _: (crate::operation::ConfigSilencerOp, crate::operation::StopOp) = r.unwrap();
    }
}
