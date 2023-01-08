/*
 * File: info.rs
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
use crate::{
    CPUControlFlags, TxDatagram, MSG_RD_CPU_VERSION, MSG_RD_FPGA_FUNCTION, MSG_RD_FPGA_VERSION,
};
use anyhow::Result;

#[derive(Default)]
pub struct CPUVersion {}

impl Operation for CPUVersion {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_RD_CPU_VERSION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x02).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
        Ok(())
    }

    fn init(&mut self) {}

    fn is_finished(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct FPGAVersion {}

impl Operation for FPGAVersion {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_RD_FPGA_VERSION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x04).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
        Ok(())
    }

    fn init(&mut self) {}

    fn is_finished(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct FPGAFunctions {}

impl Operation for FPGAFunctions {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_RD_FPGA_FUNCTION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x05).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
        Ok(())
    }

    fn init(&mut self) {}

    fn is_finished(&self) -> bool {
        true
    }
}
