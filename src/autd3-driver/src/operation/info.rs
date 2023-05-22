/*
 * File: info.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    CPUControlFlags, TxDatagram, MSG_RD_CPU_VERSION, MSG_RD_CPU_VERSION_MINOR,
    MSG_RD_FPGA_FUNCTION, MSG_RD_FPGA_VERSION, MSG_RD_FPGA_VERSION_MINOR,
};

#[derive(Default)]
pub struct CPUVersionMajor {}

impl CPUVersionMajor {
    pub fn pack(&mut self, tx: &mut TxDatagram) {
        tx.header_mut().msg_id = MSG_RD_CPU_VERSION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x02).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
    }
}

#[derive(Default)]
pub struct CPUVersionMinor {}

impl CPUVersionMinor {
    pub fn pack(&mut self, tx: &mut TxDatagram) {
        tx.header_mut().msg_id = MSG_RD_CPU_VERSION_MINOR;
        tx.num_bodies = 0;
    }
}

#[derive(Default)]
pub struct FPGAVersionMajor {}

impl FPGAVersionMajor {
    pub fn pack(&mut self, tx: &mut TxDatagram) {
        tx.header_mut().msg_id = MSG_RD_FPGA_VERSION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x04).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
    }
}

#[derive(Default)]
pub struct FPGAVersionMinor {}

impl FPGAVersionMinor {
    pub fn pack(&mut self, tx: &mut TxDatagram) {
        tx.header_mut().msg_id = MSG_RD_FPGA_VERSION_MINOR;
        tx.num_bodies = 0;
    }
}

#[derive(Default)]
pub struct FPGAFunctions {}

impl FPGAFunctions {
    pub fn pack(&mut self, tx: &mut TxDatagram) {
        tx.header_mut().msg_id = MSG_RD_FPGA_FUNCTION;
        tx.header_mut().cpu_flag = CPUControlFlags::from_bits(0x05).unwrap(); // For backward compatibility before 1.9
        tx.num_bodies = 0;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const NUM_TRANS_IN_UNIT: usize = 249;

    #[test]
    fn cpu_version() {
        let mut tx = TxDatagram::new(&[
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
        ]);

        CPUVersionMajor {}.pack(&mut tx);
        assert_eq!(tx.header().msg_id, MSG_RD_CPU_VERSION);
        assert_eq!(tx.header().cpu_flag.bits(), 0x02);

        CPUVersionMinor {}.pack(&mut tx);
        assert_eq!(tx.header().msg_id, MSG_RD_CPU_VERSION_MINOR);
    }

    #[test]
    fn fpga_version() {
        let mut tx = TxDatagram::new(&[
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
        ]);

        FPGAVersionMajor {}.pack(&mut tx);
        assert_eq!(tx.header().msg_id, MSG_RD_FPGA_VERSION);
        assert_eq!(tx.header().cpu_flag.bits(), 0x04);

        FPGAVersionMinor {}.pack(&mut tx);
        assert_eq!(tx.header().msg_id, MSG_RD_FPGA_VERSION_MINOR);
    }

    #[test]
    fn fpga_functions() {
        let mut tx = TxDatagram::new(&[
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
        ]);

        FPGAFunctions {}.pack(&mut tx);
        assert_eq!(tx.header().msg_id, MSG_RD_FPGA_FUNCTION);
        assert_eq!(tx.header().cpu_flag.bits(), 0x05);
    }
}
