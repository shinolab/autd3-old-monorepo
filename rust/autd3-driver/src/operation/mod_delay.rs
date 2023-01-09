/*
 * File: mod_delay.rs
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

use super::Operation;
use crate::{CPUControlFlags, DriverError, TxDatagram};
use anyhow::Result;

#[derive(Default)]
pub struct ModDelay {
    sent: bool,
    pub delays: Vec<u16>,
}

impl Operation for ModDelay {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        if self.is_finished() {
            return Ok(());
        }

        if self.delays.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.delays.len(),
            }
            .into());
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::MOD_DELAY, true);
        tx.num_bodies = tx.num_devices();

        tx.body_raw_mut().clone_from_slice(&self.delays);

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
