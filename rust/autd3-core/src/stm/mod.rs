/*
 * File: mod.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod gain;
mod point;

pub use gain::GainSTM;
pub use point::PointSTM;

pub trait STM {
    fn set_freq(&mut self, freq: f64) -> f64;
    fn freq(&self) -> f64;
    fn sampling_freq(&self) -> f64;
    fn set_sampling_freq_div(&mut self, freq_div: u32);
    fn sampling_freq_div(&mut self) -> u32;
}
