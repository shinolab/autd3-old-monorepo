/*
 * File: static.rs
 * Project: modulation
 * Created Date: 30/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, modulation::Modulation};
use autd3_traits::Modulation;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation)]
pub struct Static {
    amp: f64,
    freq_div: u32,
}

impl Static {
    /// constructor.
    pub fn with_amp(amp: f64) -> Self {
        Self {
            amp,
            freq_div: 40960,
        }
    }

    /// constructor.
    pub fn new() -> Self {
        Self::with_amp(1.0)
    }
}

impl Modulation for Static {
    fn calc(&self) -> Result<Vec<f64>, AUTDInternalError> {
        Ok(vec![self.amp; 2])
    }
}

impl Default for Static {
    fn default() -> Self {
        Self::new()
    }
}
