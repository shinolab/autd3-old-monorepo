/*
 * File: sync.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use super::Operation;
use crate::{CPUControlFlags, DriverError, TxDatagram};

pub trait SyncOP {}

#[derive(Default)]
pub struct SyncLegacy {
    sent: bool,
}

impl SyncOP for SyncLegacy {}

impl Operation for SyncLegacy {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        if self.is_finished() {
            return Ok(());
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SILENCER);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::CONFIG_SYNC, true);
        tx.num_bodies = tx.num_devices();

        tx.body_raw_mut().fill(4096);

        self.sent = true;
        Ok(())
    }

    fn init(&mut self) {
        self.sent = false;
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}

#[derive(Default)]
pub struct SyncNormal {
    sent: bool,
    pub cycles: Vec<u16>,
}

impl SyncOP for SyncNormal {}

impl Operation for SyncNormal {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        if self.is_finished() {
            return Ok(());
        }

        if self.cycles.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.cycles.len(),
            }
            .into());
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SILENCER);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::CONFIG_SYNC, true);
        tx.num_bodies = tx.num_devices();

        tx.body_raw_mut().clone_from_slice(&self.cycles);

        Ok(())
    }

    fn init(&mut self) {
        self.sent = false;
        self.cycles.clear();
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}
