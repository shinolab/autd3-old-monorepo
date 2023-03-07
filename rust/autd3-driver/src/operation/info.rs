/*
 * File: info.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    CPUControlFlags, TxDatagram, MSG_RD_CPU_VERSION, MSG_RD_CPU_VERSION_MINOR,
    MSG_RD_FPGA_FUNCTION, MSG_RD_FPGA_VERSION, MSG_RD_FPGA_VERSION_MINOR,
};
use anyhow::Result;

#[derive(Default)]
pub struct CPUVersionMajor {}

impl CPUVersionMajor {
    pub fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_RD_CPU_VERSION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x02).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
        Ok(())
    }
}

#[derive(Default)]
pub struct CPUVersionMinor {}

impl CPUVersionMinor {
    pub fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_RD_CPU_VERSION_MINOR;
        tx.num_bodies = 0;
        Ok(())
    }
}

#[derive(Default)]
pub struct FPGAVersionMajor {}

impl FPGAVersionMajor {
    pub fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_RD_FPGA_VERSION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x04).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
        Ok(())
    }
}

#[derive(Default)]
pub struct FPGAVersionMinor {}

impl FPGAVersionMinor {
    pub fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_RD_FPGA_VERSION_MINOR;
        tx.num_bodies = 0;
        Ok(())
    }
}

#[derive(Default)]
pub struct FPGAFunctions {}

impl FPGAFunctions {
    pub fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_RD_FPGA_FUNCTION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x05).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
        Ok(())
    }
}
