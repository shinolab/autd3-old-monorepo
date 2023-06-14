/*
 * File: fir.rs
 * Project: modulation
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

/*
 * sdr_rs module is downloaded from https://github.com/adamgreig/sdr-rs, which is distributed under the MIT License.
 * This crate is not maintained, so minor modifications have been made to make it work with the latest version of Rust.
 */
mod sdr_rs;

use sdr_rs::fir;

use autd3_core::{error::AUTDInternalError, float, modulation::Modulation};
use autd3_traits::Modulation;

#[derive(Modulation)]
pub struct ModulationFIR<M: Modulation> {
    m: M,
    freq_div: u32,
    fir: sdr_rs::fir::FIR<f32>,
}

pub trait FIR<M: Modulation> {
    fn with_low_pass(self, n_taps: usize, cutoff: float) -> ModulationFIR<M>;
    fn with_high_pass(self, n_taps: usize, cutoff: float) -> ModulationFIR<M>;
    fn with_band_pass(self, n_taps: usize, f_low: float, f_high: float) -> ModulationFIR<M>;
    fn with_band_stop(self, n_taps: usize, f_low: float, f_high: float) -> ModulationFIR<M>;
    fn with_resampler(self, n_taps: usize, decimate: usize, interpolate: usize)
        -> ModulationFIR<M>;
}

impl<M: Modulation> FIR<M> for M {
    fn with_low_pass(self, n_taps: usize, cutoff: float) -> ModulationFIR<M> {
        ModulationFIR {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::lowpass(n_taps, cutoff / self.sampling_frequency()),
            m: self,
        }
    }

    fn with_high_pass(self, n_taps: usize, cutoff: f64) -> ModulationFIR<M> {
        ModulationFIR {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::highpass(n_taps, cutoff / self.sampling_frequency()),
            m: self,
        }
    }

    fn with_band_pass(self, n_taps: usize, f_low: f64, f_high: f64) -> ModulationFIR<M> {
        ModulationFIR {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::bandpass(
                n_taps,
                f_low / self.sampling_frequency(),
                f_high / self.sampling_frequency(),
            ),
            m: self,
        }
    }

    fn with_band_stop(self, n_taps: usize, f_low: f64, f_high: f64) -> ModulationFIR<M> {
        ModulationFIR {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::bandstop(
                n_taps,
                f_low / self.sampling_frequency(),
                f_high / self.sampling_frequency(),
            ),
            m: self,
        }
    }

    fn with_resampler(
        self,
        n_taps: usize,
        decimate: usize,
        interpolate: usize,
    ) -> ModulationFIR<M> {
        ModulationFIR {
            freq_div: self.sampling_frequency_division(),
            fir: fir::FIR::resampler(n_taps, decimate, interpolate),
            m: self,
        }
    }
}

impl<M: Modulation> Modulation for ModulationFIR<M> {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        let m = self.m.calc()?;
        let coeff = self.fir.taps();
        Ok((0..m.len())
            .map(|i| {
                m.iter()
                    .cycle()
                    .skip(i)
                    .take(coeff.len())
                    .zip(coeff.iter())
                    .map(|(x, y)| x * y)
                    .sum()
            })
            .collect())
    }
}
