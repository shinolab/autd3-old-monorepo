/*
 * File: info.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    operation::{Operation, TypeTag},
};

#[derive(Debug)]
#[repr(u8)]
pub enum FirmwareInfoType {
    CPUVersionMajor = 0x01,
    CPUVersionMinor = 0x02,
    FPGAVersionMajor = 0x03,
    FPGAVersionMinor = 0x04,
    FPGAFunctions = 0x05,
}

impl From<u8> for FirmwareInfoType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::CPUVersionMajor,
            0x02 => Self::CPUVersionMinor,
            0x03 => Self::FPGAVersionMajor,
            0x04 => Self::FPGAVersionMinor,
            0x05 => Self::FPGAFunctions,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct FirmInfoOp {
    remains: HashMap<usize, usize>,
}

impl<T: Transducer> Operation<T> for FirmInfoOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        tx[0] = TypeTag::FirmwareInfo as u8;
        match self.remains[&device.idx()] {
            5 => {
                tx[1] = FirmwareInfoType::CPUVersionMajor as u8;
            }
            4 => {
                tx[1] = FirmwareInfoType::CPUVersionMinor as u8;
            }
            3 => {
                tx[1] = FirmwareInfoType::FPGAVersionMajor as u8;
            }
            2 => {
                tx[1] = FirmwareInfoType::FPGAVersionMinor as u8;
            }
            1 => {
                tx[1] = FirmwareInfoType::FPGAFunctions as u8;
            }
            _ => unreachable!(),
        }

        Ok(2)
    }

    fn required_size(&self, _: &Device<T>) -> usize {
        2
    }

    fn init(&mut self, devices: &[&Device<T>]) -> Result<(), AUTDInternalError> {
        self.remains = devices.iter().map(|device| (device.idx(), 5)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains.insert(device.idx(), self.remains(device) - 1);
    }
}

// #[cfg(test)]
// mod test {

//     use super::*;

//     const NUM_TRANS_IN_UNIT: usize = 249;

//     #[test]
//     fn cpu_version() {
//         let mut tx = TxDatagram::new(&[
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//         ]);

//         CPUVersionMajor {}.pack(&mut tx);
//         assert_eq!(tx.header().msg_id, MSG_RD_CPU_VERSION);
//         assert_eq!(tx.header().cpu_flag.bits(), 0x02);

//         CPUVersionMinor {}.pack(&mut tx);
//         assert_eq!(tx.header().msg_id, MSG_RD_CPU_VERSION_MINOR);
//     }

//     #[test]
//     fn fpga_version() {
//         let mut tx = TxDatagram::new(&[
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//         ]);

//         FPGAVersionMajor {}.pack(&mut tx);
//         assert_eq!(tx.header().msg_id, MSG_RD_FPGA_VERSION);
//         assert_eq!(tx.header().cpu_flag.bits(), 0x04);

//         FPGAVersionMinor {}.pack(&mut tx);
//         assert_eq!(tx.header().msg_id, MSG_RD_FPGA_VERSION_MINOR);
//     }

//     #[test]
//     fn fpga_functions() {
//         let mut tx = TxDatagram::new(&[
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//         ]);

//         FPGAFunctions {}.pack(&mut tx);
//         assert_eq!(tx.header().msg_id, MSG_RD_FPGA_FUNCTION);
//         assert_eq!(tx.header().cpu_flag.bits(), 0x05);
//     }
// }
