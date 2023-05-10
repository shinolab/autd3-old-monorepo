/*
 * File: silencer_config.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{error::AUTDInternalError, geometry::Transducer, sendable::Sendable};

pub struct SilencerConfig {
    step: u16,
    cycle: u16,
}

impl SilencerConfig {
    pub fn new(step: u16, cycle: u16) -> Self {
        SilencerConfig { step, cycle }
    }

    pub fn none() -> Self {
        Self::new(0xFFFF, 4096)
    }
}

impl<T: Transducer> Sendable<T> for SilencerConfig {
    type H = autd3_driver::ConfigSilencer;
    type B = autd3_driver::NullBody;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::new(self.cycle, self.step))
    }

    fn body_operation(
        &mut self,
        _geometry: &crate::geometry::Geometry<T>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::default())
    }
}

impl Default for SilencerConfig {
    fn default() -> Self {
        Self::new(10, 4096)
    }
}
