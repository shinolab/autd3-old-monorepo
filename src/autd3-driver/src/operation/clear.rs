/*
 * File: clear.rs
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

use super::Operation;
use crate::{DriverError, TxDatagram, MSG_CLEAR};

#[derive(Default)]
pub struct Clear {
    sent: bool,
}

impl Operation for Clear {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().msg_id = MSG_CLEAR;
        tx.num_bodies = 0;
        self.sent = true;
        Ok(())
    }

    fn init(&mut self) {
        self.sent = false;
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const NUM_TRANS_IN_UNIT: usize = 249;

    #[test]
    fn clear() {
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

        let mut op = Clear::default();
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());

        assert_eq!(tx.header().msg_id, MSG_CLEAR);
        assert_eq!(tx.num_bodies, 0);

        op.init();
        assert!(!op.is_finished());
    }
}
