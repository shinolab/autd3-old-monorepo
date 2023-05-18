/*
 * File: silencer_config.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{error::AUTDInternalError, geometry::*, sendable::Sendable};

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

#[cfg(not(feature = "dynamic"))]
impl<T: Transducer> Sendable<T> for SilencerConfig {
    type H = autd3_driver::ConfigSilencer;
    type B = autd3_driver::NullBody;

    fn operation(self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::new(self.step), Self::B::default()))
    }
}

#[cfg(feature = "dynamic")]
impl Sendable for SilencerConfig {
    fn operation(
        &mut self,
        _: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3_driver::Operation>,
            Box<dyn autd3_driver::Operation>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::new(autd3_driver::ConfigSilencer::new(self.step)),
            Box::new(autd3_driver::NullBody::default()),
        ))
    }
}

impl Default for SilencerConfig {
    fn default() -> Self {
        Self::new(10)
    }
}
