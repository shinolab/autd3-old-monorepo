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
};
use autd3_traits::Modulation;

use num::integer::gcd;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation, Clone, Copy)]
pub struct Square {
    freq: usize,
    low: float,
    high: float,
    duty: float,
    freq_div: u32,
}

impl Square {
    /// constructor.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave
    ///
    pub fn new(freq: usize) -> Self {
        Self::with_params(freq, 0.0, 1.0, 0.5)
    }

    /// constructor.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave
    ///
    pub fn with_params(freq: usize, low: float, high: float, duty: float) -> Self {
        Self {
            freq,
            low,
            high,
            duty,
            freq_div: 5120,
        }
    }
}

impl Modulation for Square {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        let sf = self.sampling_freq() as usize;
        let freq = self.freq.clamp(1, sf / 2);
        let k = gcd(sf, freq);
        let n = sf / k;
        let d = freq / k;

        let mut buffer = vec![self.low; n];
        let mut cursor = 0;
        for i in 0..d {
            let size = (n + i) / d;
            buffer[cursor..cursor + (size as float * self.duty) as usize].fill(self.high);
            cursor += size;
        }
        Ok(buffer)
    }
}
