/*
 * File: sine_pressure.rs
 * Project: modulation
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

use anyhow::Result;
use autd3_core::modulation::Modulation;
use autd3_traits::Modulation;

use num::integer::gcd;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation)]
pub struct SinePressure {
    op: autd3_driver::Modulation,
    freq: usize,
    amp: f64,
    offset: f64,
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
            op: Default::default(),
            freq,
            amp,
            offset,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn calc(&mut self) -> Result<()> {
        let sf = self.sampling_freq() as usize;

        let freq = self.freq.clamp(1, sf / 2);

        let d = gcd(sf, freq);

        let n = sf / d;
        let rep = freq / d;

        self.op.mod_data.resize(n, 0);

        self.op.mod_data.iter_mut().enumerate().for_each(|(i, m)| {
            let amp = self.amp / 2.0 * (2.0 * PI * (rep * i) as f64 / n as f64).sin() + self.offset;
            let amp = amp.sqrt().clamp(0.0, 1.0);
            let duty = amp.asin() * 2.0 / PI * 255.0;
            *m = duty as u8
        });

        Ok(())
    }
}
