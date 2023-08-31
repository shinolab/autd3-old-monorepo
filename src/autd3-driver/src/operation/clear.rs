/*
 * File: clear.rs
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

use super::Operation;
use crate::{
    geometry::{Device, Transducer},
    AUTDInternalError, TypeTag,
};

#[derive(Default)]
pub struct ClearOp {
    remains: HashMap<usize, usize>,
}

impl<T: Transducer> Operation<T> for ClearOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);
        tx[0] = TypeTag::Clear as u8;
        Ok(1)
    }

    fn required_size(&self, _: &Device<T>) -> usize {
        1
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

//         let mut op = Clear::default();
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());

//         assert_eq!(tx.header().msg_id, MSG_CLEAR);
//         assert_eq!(tx.num_bodies, 0);

//         op.init();
//         assert!(!op.is_finished());
//     }
// }
