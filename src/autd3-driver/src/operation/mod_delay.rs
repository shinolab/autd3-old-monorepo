/*
 * File: mod_delay.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{CPUControlFlags, DriverError, TxDatagram};

#[derive(Default)]
pub struct ModDelay {
    sent: bool,
    delays: Vec<u16>,
}

impl ModDelay {
    pub fn new(delays: Vec<u16>) -> Self {
        Self {
            sent: false,
            delays,
        }
    }
}

impl Operation for ModDelay {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        if self.is_finished() {
            return Ok(());
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::MOD_DELAY, true);
        tx.num_bodies = tx.num_devices();

        if self.delays.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.delays.len(),
            });
        }

        tx.body_raw_mut().clone_from_slice(&self.delays);

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
    fn mod_delay() {
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

        let delays = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| rng.gen_range(0x0000..0xFFFFu16))
            .collect::<Vec<_>>();

        let mut op = ModDelay::new(delays.clone());
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD_DELAY));
        tx.body_raw_mut()
            .iter()
            .zip(delays.iter())
            .for_each(|(d, delay)| {
                assert_eq!(d, delay);
            });
        assert_eq!(tx.num_bodies, 10);

        op.init();
        assert!(!op.is_finished());
    }
}
