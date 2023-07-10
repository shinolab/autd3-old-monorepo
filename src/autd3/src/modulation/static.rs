/*
 * File: static.rs
 * Project: modulation
 * Created Date: 30/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
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
    pub fn new() -> Self {
        Self {
            amp: 1.0,
            freq_div: 5120,
        }
    }

    /// set amplitude
    ///
    /// # Arguments
    ///
    /// * `amp` - peek to peek amplitude of the wave (Maximum value is 1.0)
    ///
    pub fn with_amp(self, amp: float) -> Self {
        Self { amp, ..self }
    }

    pub fn amp(&self) -> float {
        self.amp
    }
}

impl Modulation for Static {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(vec![self.amp; 2])
    }
}

impl Default for Static {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_default() {
        let mut m = Static::default();
        assert_approx_eq::assert_approx_eq!(m.amp, 1.0);
        assert_eq!(m.calc().unwrap(), vec![1.0, 1.0]);
    }

    #[test]
    fn test_static_new() {
        let mut m = Static::new();
        assert_approx_eq::assert_approx_eq!(m.amp, 1.0);
        assert_eq!(m.calc().unwrap(), vec![1.0, 1.0]);
    }

    #[test]
    fn test_static_with_amp() {
        let mut m = Static::new().with_amp(0.5);
        assert_approx_eq::assert_approx_eq!(m.amp, 0.5);
        assert_eq!(m.calc().unwrap(), vec![0.5, 0.5]);
    }
}
