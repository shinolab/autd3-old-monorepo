/*
 * File: silencer_config.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

pub struct SilencerConfig {
    step: u16,
}

impl SilencerConfig {
    pub fn new(step: u16) -> Self {
        SilencerConfig { step }
    }

    pub fn none() -> Self {
        Self::new(0xFFFF)
    }
}

impl<T: Transducer> Datagram<T> for SilencerConfig {
    type H = autd3_driver::ConfigSilencer;
    type B = autd3_driver::NullBody;

    fn operation(&mut self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::new(self.step), Self::B::default()))
    }
}

impl Default for SilencerConfig {
    fn default() -> Self {
        Self::new(10)
    }
}
