/*
 * File: clear.rs
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

use std::time::Duration;

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram for clear all data in devices
#[derive(Default)]
pub struct Clear {}

impl Clear {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for Clear {
    type O1 = crate::operation::ClearOp;
    type O2 = crate::operation::NullOp;

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::default(), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{ClearOp, NullOp};

    use super::*;

    #[test]
    fn test_clear_timeout() {
        let clear = Clear::new();
        let timeout = <Clear as Datagram<LegacyTransducer>>::timeout(&clear);
        assert!(timeout.is_some());
        assert!(timeout.unwrap() > Duration::ZERO);
    }

    #[test]
    fn test_clear_operation() {
        let clear = Clear::default();
        let r = <Clear as Datagram<LegacyTransducer>>::operation(clear);
        assert!(r.is_ok());
        let _: (ClearOp, NullOp) = r.unwrap();
    }
}
