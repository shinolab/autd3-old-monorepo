/*
 * File: sine_pressure.rs
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

use num::integer::gcd;

/// Sine wave modulation in ultrasound amplitude
#[derive(Modulation, Clone, Copy)]
pub struct SinePressure {
    freq: usize,
    amp: float,
    offset: float,
    freq_div: u32,
}

impl SinePressure {
    /// constructor.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the sine wave
    ///
    pub fn new(freq: usize) -> Self {
        Self {
            freq,
            amp: 1.0,
            offset: 0.5,
            freq_div: 5120,
        }
    }

    /// Set amplitude
    ///
    /// # Arguments
    ///
    /// * `amp` - peek to peek amplitude of the wave (Maximum value is 1.0)
    /// * `offset` - Offset of the wave
    ///
    pub fn with_amp(self, amp: float) -> Self {
        Self { amp, ..self }
    }

    /// Set offset
    ///
    /// # Arguments
    ///
    /// * `offset` - Offset of the wave
    ///
    pub fn with_offset(self, offset: float) -> Self {
        Self { offset, ..self }
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
}

impl Modulation for SinePressure {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        let sf = self.sampling_frequency() as usize;
        let freq = self.freq.clamp(1, sf / 2);
        let d = gcd(sf, freq);
        let n = sf / d;
        let rep = freq / d;

        Ok((0..n)
            .map(|i| {
                (self.amp / 2.0 * (2.0 * PI * (rep * i) as float / n as float).sin() + self.offset)
                    .sqrt()
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_pressure() {
        let expect = [
            0.7071067811865476,
            0.785316930880745,
            0.8526401643540922,
            0.9081431738250813,
            0.9510565162951535,
            0.9807852804032304,
            0.996917333733128,
            0.999229036240723,
            0.9876883405951378,
            0.9624552364536473,
            0.9238795325112867,
            0.8724960070727971,
            0.8090169943749475,
            0.7343225094356856,
            0.6494480483301837,
            0.5555702330196023,
            0.45399049973954686,
            0.34611705707749324,
            0.2334453638559055,
            0.11753739745783798,
            0.0,
            0.11753739745783774,
            0.23344536385590525,
            0.3461170570774931,
            0.45399049973954664,
            0.555570233019602,
            0.6494480483301835,
            0.7343225094356852,
            0.8090169943749473,
            0.872496007072797,
            0.9238795325112867,
            0.9624552364536472,
            0.9876883405951378,
            0.999229036240723,
            0.996917333733128,
            0.9807852804032304,
            0.9510565162951535,
            0.9081431738250815,
            0.8526401643540925,
            0.785316930880745,
            0.7071067811865477,
            0.6190939493098343,
            0.5224985647159487,
            0.4186597375374281,
            0.30901699437494756,
            0.19509032201612872,
            0.0784590957278448,
            0.03925981575906798,
            0.1564344650402306,
            0.2714404498650747,
            0.38268343236508917,
            0.4886212414969549,
            0.5877852522924729,
            0.6788007455329413,
            0.7604059656000305,
            0.8314696123025447,
            0.8910065241883678,
            0.9381913359224839,
            0.9723699203976764,
            0.9930684569549263,
            1.0,
            0.9930684569549263,
            0.9723699203976768,
            0.9381913359224842,
            0.891006524188368,
            0.8314696123025456,
            0.7604059656000308,
            0.6788007455329418,
            0.5877852522924741,
            0.48862124149695546,
            0.38268343236508967,
            0.2714404498650752,
            0.15643446504023112,
            0.03925981575906798,
            0.0784590957278441,
            0.19509032201612814,
            0.30901699437494623,
            0.4186597375374276,
            0.5224985647159489,
            0.6190939493098332,
        ];
        let mut m = SinePressure::new(150);
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
    fn test_sine_pressure_new() {
        let mut m = SinePressure::new(100);
        assert_eq!(m.freq, 100);
        assert_approx_eq::assert_approx_eq!(m.amp, 1.0);
        assert_approx_eq::assert_approx_eq!(m.offset, 0.5);

        let vec = m.calc().unwrap();
        assert!(vec.len() > 0);
        assert!(
            vec.iter()
                .all(|&x| x >= (m.offset - m.amp / 2.0).sqrt()
                    && x <= (m.offset + m.amp / 2.0).sqrt())
        );
    }

    #[test]
    fn test_sine_pressure_with_amp() {
        let mut m = SinePressure::new(100).with_amp(0.5);
        assert_approx_eq::assert_approx_eq!(m.amp, 0.5);

        let vec = m.calc().unwrap();
        assert!(vec.len() > 0);
        assert!(
            vec.iter()
                .all(|&x| x >= (m.offset - m.amp / 2.0).sqrt()
                    && x <= (m.offset + m.amp / 2.0).sqrt())
        );
    }

    #[test]
    fn test_sine_pressure_with_offset() {
        let mut m = SinePressure::new(100).with_offset(1.0);
        assert_approx_eq::assert_approx_eq!(m.offset, 1.0);

        let vec = m.calc().unwrap();
        assert!(vec.len() > 0);
        assert!(
            vec.iter()
                .all(|&x| x >= (m.offset - m.amp / 2.0).sqrt()
                    && x <= (m.offset + m.amp / 2.0).sqrt())
        );
    }
}
