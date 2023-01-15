/*
 * File: gain.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::geometry::{Geometry, Transducer};

use anyhow::Result;
use autd3_driver::Drive;

pub trait GainData {
    fn take_drives(&mut self) -> Vec<Drive>;
}

/// Gain contains amplitude and phase of each transducer in the AUTD.
/// Note that the amplitude means duty ratio of Pulse Width Modulation, respectively.
pub trait Gain<T: Transducer>: GainData {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<()>;
}
