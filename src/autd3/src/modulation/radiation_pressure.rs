/*
 * File: radiation_pressure.rs
 * Project: modulation
 * Created Date: 10/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, float, modulation::Modulation};
use autd3_derive::Modulation;

/// Modulation for modulating radiation pressure
#[derive(Modulation)]
pub struct RadiationPressureImpl<M: Modulation> {
    m: M,
    freq_div: u32,
}

pub trait RadiationPressure<M: Modulation> {
    /// Apply modulation to radiation pressure instead of amplitude
    fn with_radiation_pressure(self) -> RadiationPressureImpl<M>;
}

impl<M: Modulation> RadiationPressure<M> for M {
    fn with_radiation_pressure(self) -> RadiationPressureImpl<M> {
        RadiationPressureImpl {
            freq_div: self.sampling_frequency_division(),
            m: self,
        }
    }
}

impl<M: Modulation> Modulation for RadiationPressureImpl<M> {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.m.calc()?.iter().map(|v| v.sqrt()).collect())
    }
}
