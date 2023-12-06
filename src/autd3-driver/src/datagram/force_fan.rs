/*
 * File: force_fan.rs
 * Project: datagram
 * Created Date: 06/12/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, geometry::Device};

/// Datagram for configure force fan
pub struct ConfigureForceFan<F: Fn(&Device) -> bool> {
    f: F,
}

impl<F: Fn(&Device) -> bool> ConfigureForceFan<F> {
    /// constructor
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F: Fn(&Device) -> bool> Datagram for ConfigureForceFan<F> {
    type O1 = crate::operation::ConfigureForceFanOp<F>;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(self.f), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn f(dev: &Device) -> bool {
        dev.idx() == 0
    }

    #[test]
    fn test_force_fan_operation() {
        let datagram = ConfigureForceFan::new(f);
        let r = datagram.operation();
        assert!(r.is_ok());
        let _ = r.unwrap();
    }
}
