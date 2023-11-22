/*
 * File: custom.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    derive::{Gain, Modulation},
    driver::derive::prelude::*,
};

#[derive(Gain, Default)]
pub struct CustomGain {
    drives: HashMap<usize, Vec<Drive>>,
}

impl CustomGain {
    pub fn new() -> Self {
        Self {
            drives: Default::default(),
        }
    }

    pub fn set(self, dev_idx: usize, drives: Vec<Drive>) -> Self {
        let mut new = self;
        new.drives.insert(dev_idx, drives);
        new
    }
}

impl crate::driver::datagram::Gain for CustomGain {
    fn calc(
        &self,
        _geometry: &Geometry,
        _filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(self.drives.clone())
    }
}

#[derive(Modulation)]
pub struct CustomModulation {
    pub buf: Vec<EmitIntensity>,
    pub config: SamplingConfiguration,
}

impl crate::driver::datagram::Modulation for CustomModulation {
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}
