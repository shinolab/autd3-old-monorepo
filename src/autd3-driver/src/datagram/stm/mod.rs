/*
 * File: mod.rs
 * Project: stm
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod focus;
mod gain;

pub use focus::FocusSTM;
pub use gain::GainSTM;

use crate::{defined::float, fpga::FPGA_SUB_CLK_FREQ};

#[doc(hidden)]
/// This is used only for internal.
#[derive(Clone, Copy)]
pub struct STMProps {
    freq_div: Option<u32>,
    freq: float,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
}

impl STMProps {
    pub fn new(freq: float) -> Self {
        Self {
            freq_div: None,
            freq,
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn with_sampling_frequency_division(freq_div: u32) -> Self {
        Self {
            freq_div: Some(freq_div),
            freq: 0.,
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn with_sampling_frequency(freq: float) -> Self {
        Self {
            freq_div: Some((FPGA_SUB_CLK_FREQ as float / freq) as u32),
            freq: 0.,
            start_idx: None,
            finish_idx: None,
        }
    }

    pub fn with_sampling_period(period: std::time::Duration) -> Self {
        Self {
            freq_div: Some(
                (FPGA_SUB_CLK_FREQ as float / 1000000000. * period.as_nanos() as float) as u32,
            ),
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

    pub fn freq(&self, size: usize) -> float {
        self.freq_div.map_or(self.freq, |div| {
            FPGA_SUB_CLK_FREQ as float / div as float / size as float
        })
    }

    pub fn period(&self, size: usize) -> std::time::Duration {
        self.freq_div.map_or(
            std::time::Duration::from_nanos((1000000000. / self.freq) as _),
            |div| {
                std::time::Duration::from_nanos(
                    (div as float / FPGA_SUB_CLK_FREQ as float * size as float * 1000000000.) as _,
                )
            },
        )
    }

    pub fn sampling_frequency(&self, size: usize) -> float {
        self.freq_div
            .map_or((self.freq * size as float) as _, |div| {
                FPGA_SUB_CLK_FREQ as float / div as float
            })
    }

    pub fn sampling_period(&self, size: usize) -> std::time::Duration {
        self.freq_div.map_or(
            std::time::Duration::from_nanos((1000000000. / self.freq / size as float) as _),
            |div| {
                std::time::Duration::from_nanos(
                    (div as float / FPGA_SUB_CLK_FREQ as float * 1000000000.) as _,
                )
            },
        )
    }

    pub fn sampling_frequency_division(&self, size: usize) -> u32 {
        self.freq_div
            .unwrap_or((FPGA_SUB_CLK_FREQ as float / (self.freq * size as float)) as _)
    }
}
