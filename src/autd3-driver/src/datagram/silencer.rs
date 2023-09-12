/*
 * File: silencer.rs
 * Project: datagram
 * Created Date: 01/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram for clear all data in devices
pub struct Silencer {
    step: u16,
}

impl Silencer {
    pub const fn new(step: u16) -> Self {
        Self { step }
    }

    pub const fn disable() -> Self {
        Self { step: 0xFFFF }
    }

    pub const fn step(&self) -> u16 {
        self.step
    }
}

impl Default for Silencer {
    fn default() -> Self {
        Self::new(10)
    }
}

impl<T: Transducer> Datagram<T> for Silencer {
    type O1 = crate::operation::ConfigSilencerOp;
    type O2 = crate::operation::NullOp;

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(self.step), Self::O2::default()))
    }
}
