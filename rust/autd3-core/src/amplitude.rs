/*
 * File: amplitude.rs
 * Project: src
 * Created Date: 07/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3_driver::Drive;

use crate::{
    geometry::{Geometry, Transducer},
    interface::{DatagramBody, Empty, Filled, Sendable},
};

pub struct Amplitudes {
    amp: f64,
    sent: bool,
}

impl Amplitudes {
    pub fn uniform(amp: f64) -> Self {
        Self { amp, sent: false }
    }

    pub fn none() -> Self {
        Self::uniform(0.0)
    }
}

impl<T> DatagramBody<T> for Amplitudes
where
    T: Transducer,
{
    fn init(&mut self) -> Result<()> {
        self.sent = false;
        Ok(())
    }

    fn pack(&mut self, geometry: &Geometry<T>, tx: &mut autd3_driver::TxDatagram) -> Result<()> {
        autd3_driver::normal_head(tx);
        if DatagramBody::<T>::is_finished(self) {
            return Ok(());
        }

        let drives: Vec<_> = geometry
            .transducers()
            .map(|tr| Drive {
                phase: 0.0,
                amp: self.amp,
                cycle: tr.cycle(),
            })
            .collect();

        autd3_driver::normal_duty_body(&drives, tx)?;
        self.sent = true;
        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}

impl<T> Sendable<T> for Amplitudes
where
    T: Transducer,
{
    type H = Empty;
    type B = Filled;

    fn init(&mut self) -> Result<()> {
        DatagramBody::<T>::init(self)
    }

    fn pack(
        &mut self,
        _msg_id: u8,
        geometry: &Geometry<T>,
        tx: &mut autd3_driver::TxDatagram,
    ) -> Result<()> {
        DatagramBody::<T>::pack(self, geometry, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<T>::is_finished(self)
    }
}
