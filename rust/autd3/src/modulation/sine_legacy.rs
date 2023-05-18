/*
 * File: sine_legacy.rs
 * Project: modulation
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    float,
    modulation::{Modulation, ModulationProperty},
    PI,
};
use autd3_traits::Modulation;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation, Clone, Copy)]
pub struct SineLegacy {
    freq: float,
    amp: float,
    offset: float,
    freq_div: u32,
}

impl SineLegacy {
    /// constructor.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave
    ///
    pub fn new(freq: float) -> Self {
        Self::with_params(freq, 1.0, 0.5)
    }

    /// constructor.
    /// Sine wave oscillate from `offset`-`amp`/2 to `offset`+`amp`/2
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave
    /// * `amp` - peek to peek amplitude of the wave (Maximum value is 1.0)
    /// * `offset` - Offset of the wave
    ///
    pub fn with_params(freq: float, amp: float, offset: float) -> Self {
        Self {
            freq,
            amp,
            offset,
            freq_div: 5120,
        }
    }
}

impl Modulation for SineLegacy {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        let sf = self.sampling_freq();
        let freq = self.freq.clamp(
            autd3_core::FPGA_SUB_CLK_FREQ as float / u32::MAX as float,
            sf / 2.0,
        );
        let n = (1.0 / freq * sf).round() as usize;
        Ok((0..n)
            .map(|i| self.amp / 2.0 * (2.0 * PI * i as float / n as float).sin() + self.offset)
            .collect())
    }
}
