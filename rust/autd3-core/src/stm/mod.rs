/*
 * File: mod.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod focus;
mod gain;

use autd3_driver::FPGA_CLK_FREQ;
pub use focus::FocusSTM;
pub use gain::GainSTM;

pub trait STM {
    fn size(&self) -> usize;
    fn set_freq(&mut self, freq: f64) -> f64 {
        let sample_freq = self.size() as f64 * freq;
        self.set_sampling_freq_div((FPGA_CLK_FREQ as f64 / sample_freq) as _);
        STM::freq(self)
    }
    fn freq(&self) -> f64 {
        self.sampling_freq() / self.size() as f64
    }
    fn sampling_freq(&self) -> f64 {
        FPGA_CLK_FREQ as f64 / self.sampling_freq_div() as f64
    }
    fn set_sampling_freq(&mut self, sample_freq: f64) -> f64 {
        self.set_sampling_freq_div((FPGA_CLK_FREQ as f64 / sample_freq) as _);
        STM::sampling_freq(self)
    }
    fn set_sampling_freq_div(&mut self, freq_div: u32);
    fn sampling_freq_div(&self) -> u32;
}
