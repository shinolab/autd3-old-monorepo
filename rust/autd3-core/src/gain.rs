/*
 * File: gain.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::Drive;

use crate::{
    datagram::DatagramBody,
    geometry::{Geometry, Transducer},
};
use anyhow::Result;

pub struct GainProps {
    pub built: bool,
    pub phase_sent: bool,
    pub duty_sent: bool,
    pub drives: Vec<Drive>,
}

impl GainProps {
    pub fn new() -> Self {
        Self {
            built: false,
            phase_sent: false,
            duty_sent: false,
            drives: vec![],
        }
    }

    pub fn init<T: Transducer>(&mut self, geometry: &Geometry<T>) {
        self.drives.clear();
        self.drives = geometry
            .transducers()
            .map(|tr| Drive {
                phase: 0.0,
                amp: 0.0,
                cycle: tr.cycle(),
            })
            .collect();
    }
}

impl Default for GainProps {
    fn default() -> Self {
        Self::new()
    }
}

/// Gain contains amplitude and phase of each transducer in the AUTD.
/// Note that the amplitude means duty ratio of Pulse Width Modulation, respectively.
pub trait Gain<T: Transducer>: DatagramBody<T> {
    fn build(&mut self, geometry: &Geometry<T>) -> Result<()>;
    fn rebuild(&mut self, geometry: &Geometry<T>) -> Result<()>;
    fn drives(&self) -> &[Drive];
    fn take_drives(self) -> Vec<Drive>;
    fn built(&self) -> bool;
}
