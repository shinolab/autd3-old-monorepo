/*
 * File: modulation.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::error::AUTDInternalError;
use autd3_driver::float;

#[cfg(not(feature = "dynamic"))]
/// Modulation contains the amplitude modulation data.
pub trait ModulationProperty {
    fn sampling_frequency_division(&self) -> u32;
    fn set_sampling_frequency_division(&mut self, freq_div: u32);
    fn sampling_freq(&self) -> float;
}

#[cfg(feature = "dynamic")]
pub trait ModulationProperty: crate::sendable::Sendable {
    fn sampling_frequency_division(&self) -> u32;
    fn set_sampling_frequency_division(&mut self, freq_div: u32);
    fn sampling_freq(&self) -> float;
}

/// Modulation contains the amplitude modulation data.
pub trait Modulation: ModulationProperty {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError>;
}
