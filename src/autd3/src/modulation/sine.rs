/*
 * File: sine.rs
 * Project: modulation
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::Modulation;
use autd3_driver::{defined::PI, derive::prelude::*};

use num::integer::gcd;

/// Sine wave modulation
#[derive(Modulation, Clone, Copy)]
pub struct Sine {
    freq: usize,
    amp: float,
    phase: float,
    offset: float,
    freq_div: u32,
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
            freq_div: 5120,
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
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        let sf = self.sampling_frequency() as usize;
        let freq = self.freq.clamp(1, sf / 2);
        let d = gcd(sf, freq);
        let n = sf / d;
        let rep = freq / d;
        Ok((0..n)
            .map(|i| {
                self.amp / 2.0 * (2.0 * PI * (rep * i) as float / n as float + self.phase).sin()
                    + self.offset
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine() {
        let expect = [
            0.5,
            0.6167226819279527,
            0.7269952498697734,
            0.8247240241650918,
            0.9045084971874737,
            0.9619397662556434,
            0.9938441702975689,
            0.998458666866564,
            0.9755282581475768,
            0.9263200821770461,
            0.8535533905932737,
            0.7612492823579744,
            0.6545084971874737,
            0.5392295478639225,
            0.42178276747988463,
            0.3086582838174552,
            0.2061073738537635,
            0.1197970171999847,
            0.054496737905816106,
            0.013815039801161777,
            0.0,
            0.013815039801161721,
            0.054496737905815995,
            0.11979701719998459,
            0.20610737385376332,
            0.3086582838174548,
            0.42178276747988447,
            0.5392295478639221,
            0.6545084971874736,
            0.7612492823579742,
            0.8535533905932737,
            0.9263200821770459,
            0.9755282581475768,
            0.998458666866564,
            0.9938441702975689,
            0.9619397662556434,
            0.9045084971874737,
            0.8247240241650922,
            0.7269952498697739,
            0.6167226819279527,
            0.5000000000000002,
            0.3832773180720477,
            0.2730047501302264,
            0.17527597583490817,
            0.09549150281252639,
            0.0380602337443568,
            0.006155829702431115,
            0.0015413331334359626,
            0.024471741852423123,
            0.07367991782295413,
            0.14644660940672577,
            0.23875071764202555,
            0.34549150281252605,
            0.46077045213607704,
            0.5782172325201147,
            0.691341716182544,
            0.7938926261462363,
            0.8802029828000151,
            0.9455032620941837,
            0.9861849601988383,
            1.0,
            0.9861849601988384,
            0.9455032620941843,
            0.8802029828000155,
            0.7938926261462368,
            0.6913417161825454,
            0.5782172325201153,
            0.4607704521360776,
            0.34549150281252744,
            0.23875071764202604,
            0.14644660940672616,
            0.07367991782295441,
            0.02447174185242329,
            0.0015413331334359626,
            0.006155829702431004,
            0.038060233744356575,
            0.09549150281252555,
            0.17527597583490773,
            0.2730047501302267,
            0.3832773180720463,
        ];
        let m = Sine::new(150);
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
    fn test_sine_new() {
        let m = Sine::new(100);
        assert_eq!(m.freq, 100);
        assert_approx_eq::assert_approx_eq!(m.amp, 1.0);
        assert_approx_eq::assert_approx_eq!(m.offset, 0.5);

        let vec = m.calc().unwrap();
        assert!(!vec.is_empty());
        assert!(vec
            .iter()
            .all(|&x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }

    #[test]
    fn test_sine_with_amp() {
        let m = Sine::new(100).with_amp(0.5);
        assert_approx_eq::assert_approx_eq!(m.amp, 0.5);

        let vec = m.calc().unwrap();
        assert!(!vec.is_empty());
        assert!(vec
            .iter()
            .all(|&x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }

    #[test]
    fn test_sine_with_offset() {
        let m = Sine::new(100).with_offset(1.0);
        assert_approx_eq::assert_approx_eq!(m.offset, 1.0);

        let vec = m.calc().unwrap();
        assert!(!vec.is_empty());
        assert!(vec
            .iter()
            .all(|&x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }

    #[test]
    fn test_sine_with_phase() {
        let m = Sine::new(100).with_phase(PI / 4.0);
        assert_approx_eq::assert_approx_eq!(m.phase, PI / 4.0);

        let vec = m.calc().unwrap();
        assert!(!vec.is_empty());
        assert!(vec
            .iter()
            .all(|&x| x >= m.offset - m.amp / 2.0 && x <= m.offset + m.amp / 2.0));
    }
}
