/*
 * File: silencer.rs
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
use crate::{CPUControlFlags, DriverError, TxDatagram, SILENCER_CYCLE_MIN};
use anyhow::Result;

#[derive(Default)]
pub struct ConfigSilencer {
    sent: bool,
    pub cycle: u16,
    pub step: u16,
}

impl Operation for ConfigSilencer {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        if self.is_finished() {
            return Ok(());
        }

        if self.cycle < SILENCER_CYCLE_MIN {
            return Err(DriverError::SilencerCycleOutOfRange(self.cycle).into());
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SYNC);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::CONFIG_SILENCER, true);

        tx.header_mut().silencer_mut().cycle = self.cycle;
        tx.header_mut().silencer_mut().step = self.step;

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
