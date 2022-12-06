/*
 * File: modulation.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use crate::datagram::DatagramHeader;

pub struct ModProps {
    pub buffer: Vec<u8>,
    pub freq_div: u32,
    pub built: bool,
    pub sent: usize,
}

impl ModProps {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            freq_div: 40960,
            built: false,
            sent: 0,
        }
    }
}

impl Default for ModProps {
    fn default() -> Self {
        Self::new()
    }
}

/// Modulation contains the amplitude modulation data.
pub trait Modulation: DatagramHeader {
    fn build(&mut self) -> Result<()>;
    fn rebuild(&mut self) -> Result<()>;
    fn buffer(&self) -> &[u8];
    fn sampling_frequency_division(&mut self) -> &mut u32;
    fn sampling_freq(&self) -> f64;
}
