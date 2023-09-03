/*
 * File: fir.rs
 * Project: modulation
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

/*
 * sdr_rs module is downloaded from https://github.com/adamgreig/sdr-rs, which is distributed under the MIT License.
 * This crate is not maintained, so minor modifications have been made to make it work with the latest version of Rust.
 */
#[allow(clippy::all)]
mod sdr_rs;

use sdr_rs::fir;

use autd3_derive::Modulation;
use autd3_driver::{defined::float, error::AUTDInternalError, modulation::Modulation};

/// Modulation to apply FIR filter
#[derive(Modulation)]
pub struct FIRImpl<M: Modulation> {
    m: M,
    freq_div: u32,
    fir: sdr_rs::fir::FIR<f32>,
}

pub trait FIR<M: Modulation> {
    /// Apply low pass filter
    ///
    /// # Arguments
    ///
    /// * `n_taps` - Number of taps
    /// * `cutoff` - Cutoff frequency
    ///
    fn with_low_pass(self, n_taps: usize, cutoff: float) -> FIRImpl<M>;
    /// Apply high pass filter
    ///
    /// # Arguments
    ///
    /// * `n_taps` - Number of taps
    /// * `cutoff` - Cutoff frequency
    ///
    fn with_high_pass(self, n_taps: usize, cutoff: float) -> FIRImpl<M>;
    /// Apply band pass filter
    ///
    /// # Arguments
    ///
    /// * `n_taps` - Number of taps
    /// * `f_low` - Lower cutoff frequency
    /// * `f_high` - Higher cutoff frequency
    ///
    fn with_band_pass(self, n_taps: usize, f_low: float, f_high: float) -> FIRImpl<M>;
    /// Apply band stop filter
    ///
    /// # Arguments
    ///
    /// * `n_taps` - Number of taps
    /// * `f_low` - Lower cutoff frequency
    /// * `f_high` - Higher cutoff frequency
    ///
    fn with_band_stop(self, n_taps: usize, f_low: float, f_high: float) -> FIRImpl<M>;
}

impl<M: Modulation> FIR<M> for M {
    #[allow(clippy::unnecessary_cast)]
    fn with_low_pass(self, n_taps: usize, cutoff: float) -> FIRImpl<M> {
        FIRImpl {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::lowpass(n_taps, (cutoff / self.sampling_frequency()) as f64),
            m: self,
        }
    }

    #[allow(clippy::unnecessary_cast)]
    fn with_high_pass(self, n_taps: usize, cutoff: float) -> FIRImpl<M> {
        FIRImpl {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::highpass(n_taps, (cutoff / self.sampling_frequency()) as f64),
            m: self,
        }
    }

    #[allow(clippy::unnecessary_cast)]
    fn with_band_pass(self, n_taps: usize, f_low: float, f_high: float) -> FIRImpl<M> {
        FIRImpl {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::bandpass(
                n_taps,
                (f_low / self.sampling_frequency()) as f64,
                (f_high / self.sampling_frequency()) as f64,
            ),
            m: self,
        }
    }

    #[allow(clippy::unnecessary_cast)]
    fn with_band_stop(self, n_taps: usize, f_low: float, f_high: float) -> FIRImpl<M> {
        FIRImpl {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::bandstop(
                n_taps,
                (f_low / self.sampling_frequency()) as f64,
                (f_high / self.sampling_frequency()) as f64,
            ),
            m: self,
        }
    }
}

impl<M: Modulation> Modulation for FIRImpl<M> {
    #[allow(clippy::unnecessary_cast)]
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        let m = self.m.calc()?;
        let coeff = self.fir.taps();
        Ok((0..m.len())
            .map(|i| {
                let left = i as isize - (coeff.len() - 1) as isize / 2;
                let right = left + coeff.len() as isize;
                (left..right).enumerate().fold(0., |acc, (k, j)| {
                    acc + m[j.rem_euclid(m.len() as isize) as usize]
                        * coeff[coeff.len() - 1 - k] as float
                })
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use autd3_driver::defined::PI;

    use super::*;

    #[derive(Modulation)]
    pub struct TestModulation {
        data: Vec<float>,
        freq_div: u32,
    }

    impl TestModulation {
        pub fn new(data: Vec<float>) -> Self {
            Self {
                data,
                freq_div: 5120, // 4kHz
            }
        }
    }

    impl Modulation for TestModulation {
        fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
            Ok(self.data.clone())
        }
    }

    #[test]
    fn test_fir_impl_lowpass() {
        let fs = 4e3;
        let t = 1. / fs;
        let f_high = 1000.;
        let f_low = 100.;
        let mod_data: Vec<float> = (0..1000)
            .map(|i| {
                (2.0 * PI * f_high * i as float * t).sin()
                    + (2.0 * PI * f_low * i as float * t).sin()
            })
            .collect();

        let n_taps = 199;
        let fir_modulation =
            TestModulation::new(mod_data).with_low_pass(n_taps, (f_high + f_low) / 2.0);

        fir_modulation
            .calc()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, &val)| {
                assert!(
                    (val - (2.0 * PI * f_low * i as float * t).sin()).abs() < 0.1,
                    "Filtered value at index {} was not as expected. Expected {}, got {}",
                    i,
                    (2.0 * PI * f_low * i as float * t).sin(),
                    val
                );
            });
    }
}
