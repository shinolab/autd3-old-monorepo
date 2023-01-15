/*
 * File: flag.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{FPGAControlFlags, TxDatagram};

#[derive(Default)]
pub struct ForceFan {
    pub value: bool,
}

impl ForceFan {
    pub fn pack(&mut self, tx: &mut TxDatagram) {
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::FORCE_FAN, self.value);
    }
}

#[derive(Default)]
pub struct ReadsFPGAInfo {
    pub value: bool,
}

impl ReadsFPGAInfo {
    pub fn pack(&mut self, tx: &mut TxDatagram) {
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::READS_FPGA_INFO, self.value);
    }
}
