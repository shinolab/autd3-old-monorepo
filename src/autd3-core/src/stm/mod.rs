/*
 * File: mod.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

mod focus;
mod gain;

use autd3_driver::float;
pub use focus::FocusSTM;
pub use gain::GainSTM;

pub trait STM {
    fn new(freq: float) -> Self;
    fn with_sampling_freq_div(freq_div: u32) -> Self;
    fn with_sampling_freq(freq: float) -> Self;
    fn with_start_idx(self, idx: Option<u16>) -> Self;
    fn with_finish_idx(self, idx: Option<u16>) -> Self;
    fn size(&self) -> usize;
    fn freq(&self) -> float;
    fn sampling_freq(&self) -> float;
    fn sampling_freq_div(&self) -> u32;
    fn start_idx(&self) -> Option<u16>;
    fn finish_idx(&self) -> Option<u16>;
}
