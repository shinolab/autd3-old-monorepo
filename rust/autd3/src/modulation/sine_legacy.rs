/*
 * File: sine_legacy.rs
 * Project: modulation
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

use anyhow::Result;
use autd3_core::modulation::Modulation;
use autd3_traits::Modulation;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation)]
pub struct SineLegacy {
    freq: f64,
    amp: f64,
    offset: f64,
    freq_div: u32,
}

impl SineLegacy {
    /// constructor.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave
    ///
    pub fn new(freq: f64) -> Self {
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
    pub fn with_params(freq: f64, amp: f64, offset: f64) -> Self {
        Self {
            freq,
            amp,
            offset,
            freq_div: 40960,
        }
    }
}

impl Modulation for SineLegacy {
    fn calc(&self) -> Result<Vec<f64>> {
        let sf = self.sampling_freq();
        let freq = self
            .freq
            .clamp(autd3_core::FPGA_CLK_FREQ as f64 / u32::MAX as f64, sf / 2.0);
        let n = (1.0 / freq * sf).round() as usize;
        Ok((0..n)
            .map(|i| self.amp / 2.0 * (2.0 * PI * i as f64 / n as f64).sin() + self.offset)
            .collect())
    }
}
