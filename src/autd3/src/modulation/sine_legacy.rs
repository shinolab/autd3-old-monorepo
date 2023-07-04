/*
 * File: sine_legacy.rs
 * Project: modulation
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    float,
    modulation::{Modulation, ModulationProperty},
    PI,
};
use autd3_traits::Modulation;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation, Clone, Copy)]
pub struct SineLegacy {
    freq: float,
    amp: float,
    offset: float,
    freq_div: u32,
}

impl SineLegacy {
    /// constructor.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave
    ///
    pub fn new(freq: float) -> Self {
        Self {
            freq,
            amp: 1.0,
            offset: 0.5,
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

    /// set offset
    ///
    /// # Arguments
    ///
    /// * `offset` - Offset of the wave
    ///
    pub fn with_offset(self, offset: float) -> Self {
        Self { offset, ..self }
    }

    pub fn freq(&self) -> float {
        self.freq
    }

    pub fn amp(&self) -> float {
        self.amp
    }

    pub fn offset(&self) -> float {
        self.offset
    }
}

impl Modulation for SineLegacy {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        let sf = self.sampling_frequency();
        let freq = self.freq.clamp(
            autd3_core::FPGA_SUB_CLK_FREQ as float / u32::MAX as float,
            sf / 2.0,
        );
        let n = (1.0 / freq * sf).round() as usize;
        Ok((0..n)
            .map(|i| self.amp / 2.0 * (2.0 * PI * i as float / n as float).sin() + self.offset)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_legacy() {
        let expect = [
            0.5,
            0.6153079353712201,
            0.7243995901002311,
            0.8213938048432696,
            0.9010615963775219,
            0.959108053440137,
            0.9924038765061041,
            0.9991540791356341,
            0.9789947561577445,
            0.9330127018922194,
            0.8636868207865245,
            0.7747544890354031,
            0.6710100716628344,
            0.5580464570626149,
            0.4419535429373849,
            0.3289899283371659,
            0.225245510964597,
            0.1363131792134757,
            0.06698729810778076,
            0.021005243842255605,
            0.0008459208643659122,
            0.00759612349389599,
            0.04089194655986289,
            0.09893840362247802,
            0.1786061951567302,
            0.27560040989976875,
            0.3846920646287802,
        ];
        let mut m = SineLegacy::new(150.);
        assert_approx_eq::assert_approx_eq!(m.sampling_frequency(), 4e3);
        assert_eq!(expect.len(), m.calc().unwrap().len());
        expect
            .iter()
            .zip(m.calc().unwrap().iter())
            .for_each(|(e, a)| {
                assert_approx_eq::assert_approx_eq!(e, a);
            });
    }

    #[test]
    fn test_sine_legacy_new() {
        let mut m = SineLegacy::new(100.0);
        assert_approx_eq::assert_approx_eq!(m.freq, 100.0);
        assert_approx_eq::assert_approx_eq!(m.amp, 1.0);
        assert_approx_eq::assert_approx_eq!(m.offset, 0.5);

        let vec = m.calc().unwrap();
        assert!(vec.len() > 0);
        assert!(vec
            .iter()
            .all(|&x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }

    #[test]
    fn test_sine_legacy_with_amp() {
        let mut m = SineLegacy::new(100.0).with_amp(0.5);
        assert_approx_eq::assert_approx_eq!(m.amp, 0.5);

        let vec = m.calc().unwrap();
        assert!(vec.len() > 0);
        assert!(vec
            .iter()
            .all(|&x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }

    #[test]
    fn test_sine_legacy_with_offset() {
        let mut m = SineLegacy::new(100.0).with_offset(1.0);
        assert_approx_eq::assert_approx_eq!(m.offset, 1.0);

        let vec = m.calc().unwrap();
        assert!(vec.len() > 0);
        assert!(vec
            .iter()
            .all(|&x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }
}
