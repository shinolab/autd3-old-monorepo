/*
 * File: mod_delay.rs
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

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram to set modulation delay
#[derive(Default)]
pub struct ConfigureModDelay {}

impl ConfigureModDelay {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for ConfigureModDelay {
    type O1 = crate::operation::ConfigureModDelayOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::default(), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{ConfigureModDelayOp, NullOp};

    use super::*;

    #[test]
    fn test_mod_delay_timeout() {
        let delay = ConfigureModDelay::new();
        let timeout = <ConfigureModDelay as Datagram<LegacyTransducer>>::timeout(&delay);
        assert!(timeout.is_none());
    }

    #[test]
    fn test_mod_delay_operation() {
        let delay = ConfigureModDelay::default();
        let r = <ConfigureModDelay as Datagram<LegacyTransducer>>::operation(delay);
        assert!(r.is_ok());
        let _: (ConfigureModDelayOp, NullOp) = r.unwrap();
    }
}
