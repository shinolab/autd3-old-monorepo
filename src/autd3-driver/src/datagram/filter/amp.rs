/*
 * File: amp.rs
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

/// Datagram to set amplitude filter
#[derive(Default)]
pub struct ConfigureAmpFilter {}

impl ConfigureAmpFilter {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for ConfigureAmpFilter {
    type O1 = crate::operation::ConfigureAmpFilterOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::default(), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{ConfigureAmpFilterOp, NullOp};

    use super::*;

    #[test]
    fn test_amp_filter_timeout() {
        let filter = ConfigureAmpFilter::new();
        let timeout = <ConfigureAmpFilter as Datagram<LegacyTransducer>>::timeout(&filter);
        assert!(timeout.is_none());
    }

    #[test]
    fn test_amp_filter_operation() {
        let filter = ConfigureAmpFilter::default();
        let r = <ConfigureAmpFilter as Datagram<LegacyTransducer>>::operation(filter);
        assert!(r.is_ok());
        let _: (ConfigureAmpFilterOp, NullOp) = r.unwrap();
    }
}
