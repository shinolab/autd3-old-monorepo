/*
 * File: null.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    operation::Operation,
};

#[derive(Default)]
pub struct NullOp {}

impl<T: Transducer> Operation<T> for NullOp {
    fn pack(&mut self, _: &Device<T>, _: &mut [u8]) -> Result<usize, AUTDInternalError> {
        unreachable!()
    }

    fn required_size(&self, _: &Device<T>) -> usize {
        0
    }

    fn init(&mut self, _: &[&Device<T>]) -> Result<(), AUTDInternalError> {
        Ok(())
    }

    fn remains(&self, _: &Device<T>) -> usize {
        0
    }

    fn commit(&mut self, _: &Device<T>) {
        unreachable!()
    }
}

// #[cfg(test)]
// mod test {

//     use super::*;

//     const NUM_TRANS_IN_UNIT: usize = 249;

//     #[test]
//     fn null_header() {
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

//         tx.header_mut().cpu_flag.set(CPUControlFlags::MOD, true);
//         tx.header_mut()
//             .cpu_flag
//             .set(CPUControlFlags::CONFIG_SILENCER, true);
//         tx.header_mut()
//             .cpu_flag
//             .set(CPUControlFlags::CONFIG_SYNC, true);

//         let mut op = NullHeader::default();
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD));
//         assert!(!tx
//             .header()
//             .cpu_flag
//             .contains(CPUControlFlags::CONFIG_SILENCER));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));

//         assert_eq!(tx.header().size, 0);
//         assert_eq!(tx.num_bodies, 10);

//         op.init();
//         assert!(!op.is_finished());
//     }

//     #[test]
//     fn null_body() {
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

//         tx.header_mut()
//             .cpu_flag
//             .set(CPUControlFlags::WRITE_BODY, true);
//         tx.header_mut()
//             .cpu_flag
//             .set(CPUControlFlags::MOD_DELAY, true);
//         tx.num_bodies = 10;

//         let mut op = NullBody::default();
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_DELAY));
//         assert_eq!(tx.num_bodies, 0);

//         op.init();
//         assert!(!op.is_finished());
//     }
// }
