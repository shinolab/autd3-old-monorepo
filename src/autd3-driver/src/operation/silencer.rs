/*
 * File: silencer.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/09/2023
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

pub struct ConfigSilencerOp {
    remains: HashMap<usize, usize>,
    step: u16,
}

impl ConfigSilencerOp {
    pub fn new(step: u16) -> Self {
        Self {
            remains: Default::default(),
            step,
        }
    }
}

impl<T: Transducer> Operation<T> for ConfigSilencerOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);
        tx[0] = TypeTag::Silencer as u8;
        tx[2] = (self.step & 0xFF) as u8;
        tx[3] = (self.step >> 8) as u8;
        Ok(4)
    }

    fn required_size(&self, _: &Device<T>) -> usize {
        4
    }

    fn init(&mut self, device: &Device<T>) {
        self.remains.insert(device.idx(), 1);
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains.insert(device.idx(), 0);
    }
}

// #[cfg(test)]
// mod test {

//     use super::*;

//     const NUM_TRANS_IN_UNIT: usize = 249;

//     #[test]
//     fn clear() {
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

//         let mut op = ConfigSilencer::new(4);
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());

//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_EN_N));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));
//         assert!(tx
//             .header()
//             .cpu_flag
//             .contains(CPUControlFlags::CONFIG_SILENCER));

//         assert_eq!(tx.header().silencer().step, 4);

//         op.init();
//         assert!(!op.is_finished());
//     }
// }
