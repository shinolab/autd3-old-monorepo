/*
 * File: mod_delay.rs
 * Project: datagram
 * Created Date: 29/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, derive::prelude::Transducer, error::AUTDInternalError, geometry::Device};

/// Datagram to set modulation delay
pub struct ConfigureModDelay<F: Fn(&Device, &Transducer) -> u16> {
    f: F,
}

impl<F: Fn(&Device, &Transducer) -> u16> ConfigureModDelay<F> {
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F: Fn(&Device, &Transducer) -> u16> Datagram for ConfigureModDelay<F> {
    type O1 = crate::operation::ConfigureModDelayOp<F>;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(self.f), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn f(_dev: &Device, _tr: &Transducer) -> u16 {
        0
    }

    #[test]
    fn test_mod_delay_timeout() {
        let delay = ConfigureModDelay::new(f);
        let timeout = delay.timeout();
        assert!(timeout.is_none());
    }

    #[test]
    fn test_mod_delay_operation() {
        let delay = ConfigureModDelay::new(f);
        let r = delay.operation();
        assert!(r.is_ok());
    }
}
