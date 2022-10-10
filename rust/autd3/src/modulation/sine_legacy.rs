/*
 * File: sine_legacy.rs
 * Project: modulation
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

use anyhow::Result;
use autd3_core::modulation::{ModProps, Modulation};
use autd3_traits::Modulation;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation)]
pub struct SineLegacy {
    props: ModProps,
    freq: f64,
    amp: f64,
    offset: f64,
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
            props: ModProps::new(),
            freq,
            amp,
            offset,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn calc(&mut self) -> Result<()> {
        let sf = self.sampling_freq();

        let freq = self
            .freq
            .clamp(autd3_core::FPGA_CLK_FREQ as f64 / u32::MAX as f64, sf / 2.0);

        let n = (1.0 / freq * sf).round() as usize;

        self.props.buffer.resize(n, 0);

        self.props.buffer.iter_mut().enumerate().for_each(|(i, m)| {
            let amp = self.amp / 2.0 * (2.0 * PI * i as f64 / n as f64).sin() + self.offset;
            let amp = amp.clamp(0.0, 1.0);
            let duty = amp.asin() * 2.0 / PI * 255.0;
            *m = duty as u8
        });

        Ok(())
    }
}
