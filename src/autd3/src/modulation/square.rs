/*
 * File: sine.rs
 * Project: modulation
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::Modulation;
use autd3_driver::{common::EmitIntensity, derive::prelude::*};

use num::integer::gcd;

/// Square wave modulation
#[derive(Modulation, Clone, Copy)]
pub struct Square {
    freq: float,
    low: EmitIntensity,
    high: EmitIntensity,
    duty: float,
    config: SamplingConfiguration,
}

impl Square {
    /// constructor.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of the square wave [Hz]
    ///
    pub fn new(freq: float) -> Self {
        Self {
            freq,
            low: EmitIntensity::MIN,
            high: EmitIntensity::MAX,
            duty: 0.5,
            config: SamplingConfiguration::from_frequency(4e3).unwrap(),
        }
    }

    /// set low level amplitude
    ///
    /// # Arguments
    ///
    /// * `low` - low level amplitude (from 0 to 1)
    ///
    pub fn with_low<A: Into<EmitIntensity>>(self, low: A) -> Self {
        Self {
            low: low.into(),
            ..self
        }
    }

    /// set high level amplitude
    ///
    /// # Arguments
    ///
    /// * `high` - high level amplitude (from 0 to 1)
    ///     
    pub fn with_high<A: Into<EmitIntensity>>(self, high: A) -> Self {
        Self {
            high: high.into(),
            ..self
        }
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

    pub fn low(&self) -> EmitIntensity {
        self.low
    }

    pub fn high(&self) -> EmitIntensity {
        self.high
    }

    pub fn freq(&self) -> float {
        self.freq
    }
}

impl Modulation for Square {
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        if !(0.0..=1.0).contains(&self.duty) {
            return Err(AUTDInternalError::ModulationError(
                "duty must be in range from 0 to 1".to_string(),
            ));
        }

        let sf = self.sampling_config().frequency() as usize;
        let freq = (self.freq as usize).clamp(1, sf / 2);
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
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let m = Square::new(150.);
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
    fn test_square_with_low() {
        let m = Square::new(150.).with_low(EmitIntensity::new(0xFF));
        m.calc().unwrap().iter().for_each(|a| {
            assert_eq!(a.value(), 0xFF);
        });
    }

    #[test]
    fn test_square_with_high() {
        let m = Square::new(150.).with_high(EmitIntensity::new(0x00));
        m.calc().unwrap().iter().for_each(|a| {
            assert_eq!(a.value(), 0x00);
        });
    }

    #[test]
    fn test_square_with_duty() {
        let m = Square::new(150.).with_duty(0.0);
        m.calc().unwrap().iter().for_each(|a| {
            assert_eq!(a.value(), 0x00);
        });

        let m = Square::new(150.).with_duty(1.0);
        m.calc().unwrap().iter().for_each(|a| {
            assert_eq!(a.value(), 0xFF);
        });
    }

    #[test]
    fn test_square_with_duty_out_of_range() {
        let m = Square::new(150.).with_duty(-0.1);
        assert_eq!(
            m.calc(),
            Err(AUTDInternalError::ModulationError(
                "duty must be in range from 0 to 1".to_string()
            ))
        );

        let m = Square::new(150.).with_duty(0.0);
        assert!(m.calc().is_ok());

        let m = Square::new(150.).with_duty(1.0);
        assert!(m.calc().is_ok());

        let m = Square::new(150.).with_duty(1.1);
        assert_eq!(
            m.calc(),
            Err(AUTDInternalError::ModulationError(
                "duty must be in range from 0 to 1".to_string()
            ))
        );
    }
}
