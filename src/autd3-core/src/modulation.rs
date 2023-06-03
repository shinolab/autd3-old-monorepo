/*
 * File: modulation.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::error::AUTDInternalError;
use autd3_driver::float;

/// Modulation contains the amplitude modulation data.
pub trait ModulationProperty {
    fn sampling_freq(&self) -> float;
    fn sampling_frequency_division(&self) -> u32;
}

/// Modulation contains the amplitude modulation data.
pub trait Modulation: ModulationProperty {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError>;
}
