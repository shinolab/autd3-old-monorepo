/*
 * File: sync.rs
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

pub trait SyncOp: Operation {
    fn new<F: Fn() -> Vec<u16>>(cycles_fn: F) -> Self;
}

#[derive(Default)]
pub struct SyncLegacy {
    sent: bool,
}

impl SyncOp for SyncLegacy {
    fn new<F: Fn() -> Vec<u16>>(_: F) -> Self {
        Self { sent: false }
    }
}

impl Operation for SyncLegacy {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        if self.is_finished() {
            return Ok(());
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SILENCER);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::CONFIG_SYNC, true);
        tx.num_bodies = tx.num_devices();

        tx.body_raw_mut().fill(4096);

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

pub struct SyncAdvanced {
    sent: bool,
    cycles: Vec<u16>,
}

impl SyncOp for SyncAdvanced {
    fn new<F: Fn() -> Vec<u16>>(cycles_fn: F) -> Self {
        Self {
            sent: false,
            cycles: cycles_fn(),
        }
    }
}

impl Operation for SyncAdvanced {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        if self.is_finished() {
            return Ok(());
        }

        tx.num_bodies = tx.num_devices();

        if self.cycles.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.cycles.len(),
            });
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SILENCER);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::CONFIG_SYNC, true);

        tx.body_raw_mut().clone_from_slice(&self.cycles);

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
    use rand::prelude::*;

    use super::*;

    const NUM_TRANS_IN_UNIT: usize = 249;

    #[test]
    fn sync_legacy() {
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

        let cycles = vec![4096u16; NUM_TRANS_IN_UNIT * 10];

        let mut op = SyncLegacy::default();
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD));
        assert!(!tx
            .header()
            .cpu_flag
            .contains(CPUControlFlags::CONFIG_SILENCER));

        assert!(tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));
        tx.body_raw_mut()
            .iter()
            .zip(cycles.iter())
            .for_each(|(d, cycle)| {
                assert_eq!(d, cycle);
            });
        assert_eq!(tx.num_bodies, 10);

        op.init();
        assert!(!op.is_finished());
    }

    #[test]
    fn sync_advanced() {
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

        let mut rng = rand::thread_rng();

        let cycles = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| rng.gen_range(0..0xFFFFu16))
            .collect::<Vec<_>>();

        let mut op = SyncAdvanced::new(|| cycles.clone());
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD));
        assert!(!tx
            .header()
            .cpu_flag
            .contains(CPUControlFlags::CONFIG_SILENCER));

        assert!(tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));
        tx.body_raw_mut()
            .iter()
            .zip(cycles.iter())
            .for_each(|(d, cycle)| {
                assert_eq!(d, cycle);
            });
        assert_eq!(tx.num_bodies, 10);

        op.init();
        assert!(!op.is_finished());

        let mut op = SyncAdvanced::new(|| vec![1]);
        op.init();

        assert!(op.pack(&mut tx).is_err());
    }
}
