/*
 * File: null.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{CPUControlFlags, DriverError, TxDatagram};

#[derive(Default)]
pub struct NullHeader {
    sent: bool,
}

impl Operation for NullHeader {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SILENCER);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SYNC);

        tx.header_mut().size = 0;

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

#[derive(Default)]
pub struct NullBody {
    sent: bool,
}

impl Operation for NullBody {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
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

#[derive(Default)]
pub struct ExclusiveNullHeader {
    sent: bool,
}

impl Operation for ExclusiveNullHeader {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SILENCER);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SYNC);

        tx.header_mut().size = 0;

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

#[derive(Default)]
pub struct ExclusiveNullBody {
    sent: bool,
}

impl Operation for ExclusiveNullBody {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
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
    fn null_header() {
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

        tx.header_mut().cpu_flag.set(CPUControlFlags::MOD, true);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::CONFIG_SILENCER, true);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::CONFIG_SYNC, true);

        let mut op = NullHeader::default();
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD));
        assert!(!tx
            .header()
            .cpu_flag
            .contains(CPUControlFlags::CONFIG_SILENCER));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));

        assert_eq!(tx.header().size, 0);
        assert_eq!(tx.num_bodies, 10);

        op.init();
        assert!(!op.is_finished());
    }

    #[test]
    fn null_body() {
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

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::MOD_DELAY, true);
        tx.num_bodies = 10;

        let mut op = NullBody::default();
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_DELAY));
        assert_eq!(tx.num_bodies, 0);

        op.init();
        assert!(!op.is_finished());
    }
}
