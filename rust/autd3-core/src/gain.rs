/*
 * File: gain.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::marker::PhantomData;

use autd3_driver::{Drive, TxDatagram};

use crate::{
    geometry::{Geometry, Transducer},
    interface::DatagramBody,
};
use anyhow::Result;

pub struct GainProps<T: Transducer> {
    pub built: bool,
    pub phase_sent: bool,
    pub duty_sent: bool,
    pub drives: Vec<Drive>,
    _t: PhantomData<T>,
}

impl<T: Transducer> GainProps<T> {
    pub fn new() -> Self {
        Self {
            built: false,
            phase_sent: false,
            duty_sent: false,
            drives: vec![],
            _t: PhantomData,
        }
    }

    pub fn init(&mut self, geometry: &Geometry<T>) {
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

    pub fn pack_head(&mut self, tx: &mut TxDatagram) {
        T::pack_head(tx);
    }

    pub fn pack_body(&mut self, tx: &mut TxDatagram) -> Result<()> {
        T::pack_body(&mut self.phase_sent, &mut self.duty_sent, &self.drives, tx)
    }
}

impl<T: Transducer> Default for GainProps<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IGain<T: Transducer> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<()>;
}

/// Gain contains amplitude and phase of each transducer in the AUTD.
/// Note that the amplitude means duty ratio of Pulse Width Modulation, respectively.
pub trait Gain<T: Transducer>: IGain<T> + DatagramBody<T> {
    fn build(&mut self, geometry: &Geometry<T>) -> Result<()>;
    fn rebuild(&mut self, geometry: &Geometry<T>) -> Result<()>;
    fn drives(&self) -> &[Drive];
    fn take_drives(self) -> Vec<Drive>;
    fn built(&self) -> bool;
}
