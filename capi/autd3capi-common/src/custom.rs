/*
 * File: custom.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    core::{
        error::AUTDInternalError,
        float,
        gain::GainFilter,
        geometry::{Geometry, Transducer},
        Drive,
    },
    derive::{Gain, Modulation},
    Gain, Modulation,
};

#[derive(Gain)]
pub struct CustomGain {
    pub drives: Vec<Drive>,
}

impl<T: Transducer> crate::core::gain::Gain<T> for CustomGain {
    fn calc(
        &self,
        _geometry: &Geometry<T>,
        _filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(self.drives.clone())
    }
}

#[derive(Modulation)]
pub struct CustomModulation {
    pub buf: Vec<float>,
    pub freq_div: u32,
}

impl crate::core::modulation::Modulation for CustomModulation {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}
