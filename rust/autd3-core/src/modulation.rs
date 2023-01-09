/*
 * File: modulation.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::datagram::DatagramHeader;

/// Modulation contains the amplitude modulation data.
pub trait Modulation: DatagramHeader {
    fn buffer(&self) -> &[u8];
    fn sampling_frequency_division(&mut self) -> &mut u32;
    fn sampling_freq(&self) -> f64;
}
