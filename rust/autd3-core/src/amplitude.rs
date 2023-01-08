/*
 * File: amplitude.rs
 * Project: src
 * Created Date: 07/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3_driver::{Drive, Operation};

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{Geometry, NormalPhaseTransducer},
};

pub struct Amplitudes {
    amp: f64,
    op: autd3_driver::GainNormal,
}

impl Amplitudes {
    pub fn uniform(amp: f64) -> Self {
        Self {
            amp,
            op: Default::default(),
        }
    }

    pub fn none() -> Self {
        Self::uniform(0.0)
    }
}

impl DatagramBody<NormalPhaseTransducer> for Amplitudes {
    fn init(&mut self, geometry: &Geometry<NormalPhaseTransducer>) -> Result<()> {
        self.op.init();
        self.op.phase_sent = true;
        self.op.drives = (0..geometry.num_transducers())
            .map(|_| Drive {
                phase: 0.0,
                amp: self.amp,
            })
            .collect();
        self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(())
    }

    fn pack(&mut self, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl Sendable<NormalPhaseTransducer> for Amplitudes {
    type H = Empty;
    type B = Filled;

    fn init(&mut self, geometry: &Geometry<NormalPhaseTransducer>) -> Result<()> {
        DatagramBody::<NormalPhaseTransducer>::init(self, geometry)
    }

    fn pack(&mut self, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        DatagramBody::<NormalPhaseTransducer>::pack(self, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<NormalPhaseTransducer>::is_finished(self)
    }
}
