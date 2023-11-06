/*
 * File: synchronize.rs
 * Project: datagram
 * Created Date: 29/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{datagram::*, error::AUTDInternalError};

/// Datagram to synchronize devices
#[derive(Default)]
pub struct Synchronize {}

impl Synchronize {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Datagram for Synchronize {
    type O1 = crate::operation::SyncOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Default::default(), Default::default()))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_timeout() {
        let stop = Synchronize::new();
        let timeout = <Synchronize as Datagram>::timeout(&stop);
        assert!(timeout.is_some());
        assert!(timeout.unwrap() > Duration::ZERO);
    }

    #[test]
    fn test_sync_operation() {
        let stop = Synchronize::default();
        let r = <Synchronize as Datagram>::operation(stop);
        assert!(r.is_ok());
        let _: (crate::operation::SyncOp, crate::operation::NullOp) = r.unwrap();
    }
}
