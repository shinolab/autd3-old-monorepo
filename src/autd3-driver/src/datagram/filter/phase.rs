/*
 * File: phase.rs
 * Project: filter
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram to set phase filter
#[derive(Default)]
pub struct ConfigurePhaseFilter {}

impl ConfigurePhaseFilter {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for ConfigurePhaseFilter {
    type O1 = crate::operation::ConfigurePhaseFilterOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::default(), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{ConfigurePhaseFilterOp, NullOp};

    use super::*;

    #[test]
    fn test_amp_filter_timeout() {
        let filter = ConfigurePhaseFilter::new();
        let timeout = <ConfigurePhaseFilter as Datagram<LegacyTransducer>>::timeout(&filter);
        assert!(timeout.is_none());
    }

    #[test]
    fn test_amp_filter_operation() {
        let filter = ConfigurePhaseFilter::default();
        let r = <ConfigurePhaseFilter as Datagram<LegacyTransducer>>::operation(filter);
        assert!(r.is_ok());
        let _: (ConfigurePhaseFilterOp, NullOp) = r.unwrap();
    }
}
