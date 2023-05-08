/*
 * File: silencer.rs
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
use crate::{CPUControlFlags, DriverError, TxDatagram, SILENCER_CYCLE_MIN};

#[derive(Default)]
pub struct ConfigSilencer {
    sent: bool,
    cycle: u16,
    step: u16,
}

impl ConfigSilencer {
    pub fn new(cycle: u16, step: u16) -> Self {
        Self {
            sent: false,
            step,
            cycle,
        }
    }
}

impl Operation for ConfigSilencer {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        if self.is_finished() {
            return Ok(());
        }

        if self.cycle < SILENCER_CYCLE_MIN {
            return Err(DriverError::SilencerCycleOutOfRange(self.cycle));
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
        tx.header_mut()
            .cpu_flag
            .remove(CPUControlFlags::CONFIG_SYNC);
        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::CONFIG_SILENCER, true);

        tx.header_mut().silencer_mut().cycle = self.cycle;
        tx.header_mut().silencer_mut().step = self.step;

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

        let mut op = ConfigSilencer::new(1044, 4);
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());

        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_EN_N));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::CONFIG_SYNC));
        assert!(tx
            .header()
            .cpu_flag
            .contains(CPUControlFlags::CONFIG_SILENCER));

        assert_eq!(tx.header().silencer().cycle, 1044);
        assert_eq!(tx.header().silencer().step, 4);

        op.init();
        assert!(!op.is_finished());

        let mut op = ConfigSilencer::new(1043, 4);
        op.init();
        assert!(!op.is_finished());
        assert!(op.pack(&mut tx).is_err());
    }
}
