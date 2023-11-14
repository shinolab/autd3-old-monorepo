/*
 * File: radiation_pressure.rs
 * Project: modulation
 * Created Date: 10/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::Modulation;
use autd3_driver::derive::prelude::*;

/// Modulation for modulating radiation pressure
#[derive(Modulation)]
pub struct RadiationPressure<M: Modulation> {
    m: M,
    #[no_change]
    config: SamplingConfiguration,
}

pub trait IntoRadiationPressure<M: Modulation> {
    /// Apply modulation to radiation pressure instead of amplitude
    fn with_radiation_pressure(self) -> RadiationPressure<M>;
}

impl<M: Modulation> IntoRadiationPressure<M> for M {
    fn with_radiation_pressure(self) -> RadiationPressure<M> {
        RadiationPressure {
            config: self.sampling_config(),
            m: self,
        }
    }
}

impl<M: Modulation> Modulation for RadiationPressure<M> {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.m.calc()?.iter().map(|v| v.sqrt()).collect())
    }
}
