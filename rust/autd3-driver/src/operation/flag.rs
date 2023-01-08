/*
 * File: flag.rs
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
use crate::{FPGAControlFlags, TxDatagram};
use anyhow::Result;

#[derive(Default)]
pub struct ForceFan {
    pub value: bool,
}

impl Operation for ForceFan {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::FORCE_FAN, self.value);
        Ok(())
    }

    fn init(&mut self) {}

    fn is_finished(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct ReadsFPGAInfo {
    pub value: bool,
}

impl Operation for ReadsFPGAInfo {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::READS_FPGA_INFO, self.value);
        Ok(())
    }

    fn init(&mut self) {}

    fn is_finished(&self) -> bool {
        true
    }
}
