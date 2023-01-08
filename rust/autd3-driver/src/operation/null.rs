/*
 * File: null.rs
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
use crate::{CPUControlFlags, TxDatagram};

#[derive(Default)]
pub struct NullHeader {}

impl Operation for NullHeader {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SILENCER);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SYNC);

        tx.header_mut().size = 0;
        Ok(())
    }

    fn init(&mut self) {}

    fn is_finished(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct NullBody {}

impl Operation for NullBody {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
        tx.num_bodies = 0;
        Ok(())
    }

    fn init(&mut self) {}

    fn is_finished(&self) -> bool {
        true
    }
}
