/*
 * File: custom.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::common::{
    core::{
        error::AUTDInternalError,
        gain::GainFilter,
        geometry::{Geometry, Transducer},
        Drive,
    },
    float,
    traits::{Gain, Modulation},
    Gain, Modulation,
};

#[derive(Gain)]
pub struct CustomGain {
    pub drives: Vec<Drive>,
}

impl<T: Transducer> autd3capi_def::common::core::gain::Gain<T> for CustomGain {
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

impl autd3capi_def::common::core::modulation::Modulation for CustomModulation {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}
