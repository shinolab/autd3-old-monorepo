/*
 * File: sine.rs
 * Project: modulation
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
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
        Self {
            freq,
            low: 0.0,
            high: 1.0,
            duty: 0.5,
            freq_div: 5120,
        }
    }

    /// set low
    ///
    /// # Arguments
    ///
    /// * `low` - low value
    ///
    pub fn with_low(self, low: float) -> Self {
        Self { low, ..self }
    }

    /// set high
    ///
    /// # Arguments
    ///
    /// * `high` - high value
    ///     
    pub fn with_high(self, high: float) -> Self {
        Self { high, ..self }
    }

    /// set duty
    ///
    /// # Arguments
    ///     
    /// * `duty` - duty
    ///
    pub fn with_duty(self, duty: float) -> Self {
        Self { duty, ..self }
    }
}

impl Modulation for Square {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        let sf = self.sampling_frequency() as usize;
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
