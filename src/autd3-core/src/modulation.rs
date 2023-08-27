/*
 * File: modulation.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::error::AUTDInternalError;
use autd3_driver::float;

pub trait ModulationProperty {
    fn sampling_frequency(&self) -> float {
        autd3_driver::FPGA_SUB_CLK_FREQ as float / self.sampling_frequency_division() as float
    }
    fn sampling_frequency_division(&self) -> u32;
    fn sampling_period(&self) -> std::time::Duration {
        std::time::Duration::from_nanos((1000000000. / self.sampling_frequency()) as u64)
    }
}

/// Modulation controls the amplitude modulation data.
///
/// Modulation has following restrictions:
/// * The buffer size is up to 65536.
/// * The sampling rate is [crate::FPGA_SUB_CLK_FREQ]/N, where N is a 32-bit unsigned integer and must be at least [crate::SAMPLING_FREQ_DIV_MIN].
/// * Modulation is common to all devices.
/// * Modulation automatically loops. It is not possible to control only one loop, etc.
/// * The start/end timing of Modulation cannot be controlled.
pub trait Modulation: ModulationProperty {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError>;
}

impl ModulationProperty for Box<dyn Modulation> {
    fn sampling_frequency(&self) -> f64 {
        self.as_ref().sampling_frequency()
    }

    fn sampling_frequency_division(&self) -> u32 {
        self.as_ref().sampling_frequency_division()
    }

    fn sampling_period(&self) -> std::time::Duration {
        self.as_ref().sampling_period()
    }
}

impl Modulation for Box<dyn Modulation> {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        self.as_ref().calc()
    }
}
