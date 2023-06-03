/*
 * File: mod.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

mod focus;
mod gain;

use autd3_driver::{float, FPGA_SUB_CLK_FREQ};
pub use focus::{ControlPoint, FocusSTM};
pub use gain::GainSTM;

#[derive(Clone, Copy)]
pub struct STMProps {
    size: usize,
    freq_div: Option<u32>,
    freq: float,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
}

impl STMProps {
    pub fn new(freq: float) -> Self {
        Self {
            size: 0,
            freq_div: None,
            freq,
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn with_sampling_freq_div(freq_div: u32) -> Self {
        Self {
            size: 0,
            freq_div: Some(freq_div),
            freq: 0.,
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn with_sampling_freq(freq: float) -> Self {
        Self {
            size: 0,
            freq_div: Some((FPGA_SUB_CLK_FREQ as float / freq) as u32),
            freq: 0.,
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn with_start_idx(self, idx: Option<u16>) -> Self {
        Self {
            start_idx: idx,
            ..self
        }
    }

    pub fn with_finish_idx(self, idx: Option<u16>) -> Self {
        Self {
            finish_idx: idx,
            ..self
        }
    }

    pub fn start_idx(&self) -> Option<u16> {
        self.start_idx
    }

    pub fn finish_idx(&self) -> Option<u16> {
        self.finish_idx
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn freq(&self) -> f64 {
        self.freq_div.map_or(self.freq, |div| {
            FPGA_SUB_CLK_FREQ as float / div as float / self.size() as float
        })
    }

    pub fn sampling_freq(&self) -> f64 {
        self.freq_div
            .map_or((self.freq * self.size() as float) as _, |div| {
                FPGA_SUB_CLK_FREQ as float / div as float
            })
    }

    pub fn sampling_freq_div(&self) -> u32 {
        self.freq_div
            .unwrap_or((FPGA_SUB_CLK_FREQ as float / (self.freq * self.size() as float)) as _)
    }
}
