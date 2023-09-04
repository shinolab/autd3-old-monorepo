/*
 * File: sine.rs
 * Project: modulation
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::Modulation;
use autd3_driver::{
    datagram::{Modulation, ModulationProperty},
    defined::float,
    error::AUTDInternalError,
};

use num::integer::gcd;

/// Square wave modulation
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
    /// * `freq` - Frequency of the square wave
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

    /// set low level amplitude
    ///
    /// # Arguments
    ///
    /// * `low` - low level amplitude (from 0 to 1)
    ///
    pub fn with_low(self, low: float) -> Self {
        Self { low, ..self }
    }

    /// set high level amplitude
    ///
    /// # Arguments
    ///
    /// * `high` - high level amplitude (from 0 to 1)
    ///     
    pub fn with_high(self, high: float) -> Self {
        Self { high, ..self }
    }

    /// set duty ratio which is defined as `Th / (Th + Tl)`, where `Th` is high level duration, and `Tl` is low level duration.
    ///
    /// # Arguments
    ///     
    /// * `duty` - duty ratio
    ///
    pub fn with_duty(self, duty: float) -> Self {
        Self { duty, ..self }
    }

    pub fn duty(&self) -> float {
        self.duty
    }

    pub fn low(&self) -> float {
        self.low
    }

    pub fn high(&self) -> float {
        self.high
    }

    pub fn freq(&self) -> usize {
        self.freq
    }
}

impl Modulation for Square {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        let sf = self.sampling_frequency() as usize;
        let freq = self.freq.clamp(1, sf / 2);
        let k = gcd(sf, freq);
        let n = sf / k;
        let d = freq / k;
        Ok((0..d)
            .map(|i| (n + i) / d)
            .flat_map(|size| {
                let n_high = (size as float * self.duty) as usize;
                vec![self.high; n_high]
                    .into_iter()
                    .chain(vec![self.low; size - n_high])
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        let expect = [
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let m = Square::new(150);
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
    fn test_square_with_low() {
        let m = Square::new(150).with_low(1.0);
        m.calc().unwrap().iter().for_each(|a| {
            assert_approx_eq::assert_approx_eq!(a, 1.0);
        });
    }

    #[test]
    fn test_square_with_high() {
        let m = Square::new(150).with_high(0.0);
        m.calc().unwrap().iter().for_each(|a| {
            assert_approx_eq::assert_approx_eq!(a, 0.0);
        });
    }

    #[test]
    fn test_square_with_duty() {
        let m = Square::new(150).with_duty(0.0);
        m.calc().unwrap().iter().for_each(|a| {
            assert_approx_eq::assert_approx_eq!(a, 0.0);
        });

        let m = Square::new(150).with_duty(1.0);
        m.calc().unwrap().iter().for_each(|a| {
            assert_approx_eq::assert_approx_eq!(a, 1.0);
        });
    }
}
