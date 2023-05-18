/*
 * File: sine.rs
 * Project: modulation
 * Created Date: 28/04/2022
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

use num::integer::gcd;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation, Clone, Copy)]
pub struct Sine {
    freq: usize,
    amp: float,
    offset: float,
    freq_div: u32,
}

impl Sine {
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
    pub fn with_params(freq: usize, amp: float, offset: float) -> Self {
        Self {
            freq,
            amp,
            offset,
            freq_div: 5120,
        }
    }
}

impl Modulation for Sine {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        let sf = self.sampling_freq() as usize;
        let freq = self.freq.clamp(1, sf / 2);
        let d = gcd(sf, freq);
        let n = sf / d;
        let rep = freq / d;
        Ok((0..n)
            .map(|i| {
                self.amp / 2.0 * (2.0 * PI * (rep * i) as float / n as float).sin() + self.offset
            })
            .collect())
    }
}
