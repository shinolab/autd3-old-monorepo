/*
 * File: sine.rs
 * Project: modulation
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::Modulation;
use autd3_driver::{common::EmitIntensity, defined::PI, derive::prelude::*};

use num::integer::gcd;

/// Sine wave modulation
#[derive(Modulation, Clone, Copy)]
pub struct Sine {
    freq: usize,
    amp: float,
    phase: float,
    offset: float,
    config: SamplingConfiguration,
}

impl Sine {
    /// constructor
    ///
    /// The sine wave is defined as `amp / 2 * sin(2π * freq * t + phase) + offset`, where `t` is time, and `amp = 1`, `phase = 0`, `offset = 0.5` by default.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave [Hz]
    ///
    pub fn new(freq: usize) -> Self {
        Self {
            freq,
            amp: 1.0,
            phase: 0.0,
            offset: 0.5,
            config: SamplingConfiguration::new_with_frequency(4e3).unwrap(),
        }
    }

    /// set amplitude
    ///
    /// # Arguments
    ///
    /// * `amp` - peek to peek amplitude of the wave
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

    /// set phase
    ///
    /// # Arguments
    ///
    /// * `phase` - Phase of the wave
    ///
    pub fn with_phase(self, phase: float) -> Self {
        Self { phase, ..self }
    }

    pub fn freq(&self) -> usize {
        self.freq
    }

    pub fn amp(&self) -> float {
        self.amp
    }

    pub fn offset(&self) -> float {
        self.offset
    }

    pub fn phase(&self) -> float {
        self.phase
    }
}

impl Modulation for Sine {
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        let sf = self.sampling_config().frequency() as usize;
        let freq = self.freq.clamp(1, sf / 2);
        let d = gcd(sf, freq);
        let n = sf / d;
        let rep = freq / d;
        Ok((0..n)
            .map(|i| {
                self.amp / 2.0 * (2.0 * PI * (rep * i) as float / n as float + self.phase).sin()
                    + self.offset
            })
            .map(|v| EmitIntensity::new((v * 255.0).round() as u8))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine() {
        let expect = [
            128, 157, 185, 210, 231, 245, 253, 255, 249, 236, 218, 194, 167, 138, 108, 79, 53, 31,
            14, 4, 0, 4, 14, 31, 53, 79, 108, 138, 167, 194, 218, 236, 249, 255, 253, 245, 231,
            210, 185, 157, 128, 98, 70, 45, 24, 10, 2, 0, 6, 19, 37, 61, 88, 117, 147, 176, 202,
            224, 241, 251, 255, 251, 241, 224, 202, 176, 147, 117, 88, 61, 37, 19, 6, 0, 2, 10, 24,
            45, 70, 98,
        ];
        let m = Sine::new(150);
        assert_approx_eq::assert_approx_eq!(m.sampling_config().frequency(), 4e3);
        assert_eq!(expect.len(), m.calc().unwrap().len());
        expect
            .into_iter()
            .zip(m.calc().unwrap().iter())
            .for_each(|(e, a)| {
                assert_eq!(e, a.value());
            });
    }

    #[test]
    fn test_sine_new() {
        let m = Sine::new(100);
        assert_eq!(m.freq(), 100);
        assert_eq!(m.amp(), 1.0);
        assert_eq!(m.offset(), 0.5);
        assert_eq!(m.phase(), 0.0);

        let vec = m.calc().unwrap();
        assert!(!vec.is_empty());
        assert!(vec
            .iter()
            .map(|&x| x.value() as float / 255.)
            .all(|x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }

    #[test]
    fn test_sine_with_amp() {
        let m = Sine::new(100).with_amp(0.5);
        assert_eq!(m.amp, 0.5);

        let vec = m.calc().unwrap();
        assert!(!vec.is_empty());
        assert!(vec
            .iter()
            .map(|&x| x.value() as float / 255.)
            .all(|x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }

    #[test]
    fn test_sine_with_offset() {
        let m = Sine::new(100).with_offset(1.0);
        assert_eq!(m.offset, 1.0);

        let vec = m.calc().unwrap();
        assert!(!vec.is_empty());
        assert!(vec
            .iter()
            .map(|&x| x.value() as float / 255.)
            .all(|x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }

    #[test]
    fn test_sine_with_phase() {
        let m = Sine::new(100).with_phase(PI / 4.0);
        assert_eq!(m.phase, PI / 4.0);

        let vec = m.calc().unwrap();
        assert!(!vec.is_empty());
        assert!(vec
            .iter()
            .map(|&x| x.value() as float / 255.)
            .all(|x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }
}
