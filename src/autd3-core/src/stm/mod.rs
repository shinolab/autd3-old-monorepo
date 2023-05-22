/*
 * File: mod.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

mod focus;
mod gain;

use autd3_driver::{float, FPGA_SUB_CLK_FREQ};
pub use focus::FocusSTM;
pub use gain::GainSTM;

pub trait STM {
    fn size(&self) -> usize;
    fn set_freq(&mut self, freq: float) -> float {
        let sample_freq = self.size() as float * freq;
        self.set_sampling_freq_div((FPGA_SUB_CLK_FREQ as float / sample_freq) as _);
        STM::freq(self)
    }
    fn freq(&self) -> float {
        self.sampling_freq() / self.size() as float
    }
    fn sampling_freq(&self) -> float {
        FPGA_SUB_CLK_FREQ as float / self.sampling_freq_div() as float
    }
    fn set_sampling_freq(&mut self, sample_freq: float) -> float {
        self.set_sampling_freq_div((FPGA_SUB_CLK_FREQ as float / sample_freq) as _);
        STM::sampling_freq(self)
    }
    fn set_sampling_freq_div(&mut self, freq_div: u32);
    fn sampling_freq_div(&self) -> u32;
    fn start_idx(&self) -> Option<u16>;
    fn finish_idx(&self) -> Option<u16>;
    fn set_start_idx(&mut self, idx: Option<u16>);
    fn set_finish_idx(&mut self, idx: Option<u16>);
}
