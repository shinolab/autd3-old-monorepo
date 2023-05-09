/*
 * File: sine_pressure.rs
 * Project: modulation
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

use autd3_core::{error::AUTDInternalError, modulation::Modulation};
use autd3_traits::Modulation;

use num::integer::gcd;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation)]
pub struct SinePressure {
    freq: usize,
    amp: f64,
    offset: f64,
    freq_div: u32,
}

impl SinePressure {
    /// constructor.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave
    ///
    pub fn new(freq: usize) -> Self {
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
    pub fn with_params(freq: usize, amp: f64, offset: f64) -> Self {
        Self {
            freq,
            amp,
            offset,
            freq_div: 40960,
        }
    }
}

impl Modulation for SinePressure {
    fn calc(&self) -> Result<Vec<f64>, AUTDInternalError> {
        let sf = self.sampling_freq() as usize;
        let freq = self.freq.clamp(1, sf / 2);
        let d = gcd(sf, freq);
        let n = sf / d;
        let rep = freq / d;

        Ok((0..n)
            .map(|i| self.amp / 2.0 * (2.0 * PI * (rep * i) as f64 / n as f64).sin() + self.offset)
            .collect())
    }
}
