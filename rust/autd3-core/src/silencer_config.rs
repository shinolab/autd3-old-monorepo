/*
 * File: silencer_config.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::{DatagramHeader, Empty, Filled, Sendable},
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

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

impl DatagramHeader for SilencerConfig {
    type O = autd3_driver::ConfigSilencer;

    fn operation(&mut self) -> Result<Self::O, AUTDInternalError> {
        Ok(autd3_driver::ConfigSilencer::new(self.step, self.cycle))
    }
}

impl<T: Transducer> Sendable<T> for SilencerConfig {
    type H = Filled;
    type B = Empty;
    type O = <Self as DatagramHeader>::O;

    fn operation(&mut self, _: &Geometry<T>) -> Result<Self::O, AUTDInternalError> {
        <Self as DatagramHeader>::operation(self)
    }
}

impl Default for SilencerConfig {
    fn default() -> Self {
        Self::new(10, 4096)
    }
}
