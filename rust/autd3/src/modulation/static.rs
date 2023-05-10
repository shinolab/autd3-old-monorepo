/*
 * File: static.rs
 * Project: modulation
 * Created Date: 30/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, float, modulation::Modulation};
use autd3_traits::Modulation;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation, Clone, Copy)]
pub struct Static {
    amp: float,
    freq_div: u32,
}

impl Static {
    /// constructor.
    pub fn with_amp(amp: float) -> Self {
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
    fn calc(self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(vec![self.amp; 2])
    }
}

impl Default for Static {
    fn default() -> Self {
        Self::new()
    }
}
