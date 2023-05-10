/*
 * File: modulation.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{
    float, CPUControlFlags, DriverError, TxDatagram, MOD_BUF_SIZE_MAX,
    MOD_HEADER_INITIAL_DATA_SIZE, MOD_HEADER_SUBSEQUENT_DATA_SIZE, MOD_SAMPLING_FREQ_DIV_MIN, PI,
};

pub struct Modulation {
    mod_data: Vec<u8>,
    sent: usize,
    freq_div: u32,
}

impl Modulation {
    pub fn new(mod_data: Vec<float>, freq_div: u32) -> Self {
        Self {
            mod_data: mod_data.into_iter().map(Self::to_duty).collect(),
            sent: 0,
            freq_div,
        }
    }

    pub fn to_duty(amp: float) -> u8 {
        (amp.clamp(0., 1.).asin() * 2.0 / PI * 255.0) as u8
    }
}

impl Operation for Modulation {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        if self.mod_data.len() > MOD_BUF_SIZE_MAX {
            return Err(DriverError::ModulationSizeOutOfRange(self.mod_data.len()));
        }

        let is_first_frame = self.sent == 0;
        let max_size = if is_first_frame {
            MOD_HEADER_INITIAL_DATA_SIZE
        } else {
            MOD_HEADER_SUBSEQUENT_DATA_SIZE
        };
        let mod_size = (self.mod_data.len() - self.sent).min(max_size);
        if mod_size == 0 {
            tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD);
            return Ok(());
        }
        let is_last_frame = self.sent + mod_size == self.mod_data.len();

        tx.header_mut().cpu_flag.set(CPUControlFlags::MOD, true);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_BEGIN);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_END);
        tx.header_mut().size = mod_size as _;

        if is_first_frame {
            if self.freq_div < MOD_SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::ModFreqDivOutOfRange(self.freq_div));
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::MOD_BEGIN, true);
            tx.header_mut().mod_initial_mut().freq_div = self.freq_div;
            tx.header_mut().mod_initial_mut().data[0..mod_size]
                .copy_from_slice(&self.mod_data[self.sent..self.sent + mod_size]);
        } else {
            tx.header_mut().mod_subsequent_mut().data[0..mod_size]
                .copy_from_slice(&self.mod_data[self.sent..self.sent + mod_size]);
        }

        if is_last_frame {
            tx.header_mut().cpu_flag.set(CPUControlFlags::MOD_END, true);
        }

        self.sent += mod_size;

        Ok(())
    }

    fn init(&mut self) {
        self.sent = 0;
    }

    fn is_finished(&self) -> bool {
        self.sent == self.mod_data.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::float;

    const NUM_TRANS_IN_UNIT: usize = 249;

    #[test]
    fn modulation() {
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

        let size = MOD_HEADER_INITIAL_DATA_SIZE + MOD_HEADER_SUBSEQUENT_DATA_SIZE + 1;
        let mod_data = (0..size)
            .map(|i| (i as float) / (size as float))
            .collect::<Vec<_>>();

        let mut op = Modulation::new(mod_data.clone(), 1160);
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
        assert_eq!(tx.header().size as usize, MOD_HEADER_INITIAL_DATA_SIZE);
        assert_eq!(tx.header().mod_initial().freq_div, 1160);
        for (&h, &d) in tx.header().mod_initial().data.iter().zip(mod_data.iter()) {
            assert_eq!(h, Modulation::to_duty(d));
        }

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
        assert_eq!(tx.header().size as usize, MOD_HEADER_SUBSEQUENT_DATA_SIZE);
        for i in 0..MOD_HEADER_SUBSEQUENT_DATA_SIZE {
            assert_eq!(
                tx.header().mod_subsequent().data[i],
                Modulation::to_duty(mod_data[MOD_HEADER_INITIAL_DATA_SIZE + i])
            );
        }

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::MOD_BEGIN));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::MOD_END));
        assert_eq!(tx.header().size as usize, 1);
        for i in 0..1 {
            assert_eq!(
                tx.header().mod_subsequent().data[i],
                Modulation::to_duty(
                    mod_data[MOD_HEADER_INITIAL_DATA_SIZE + MOD_HEADER_SUBSEQUENT_DATA_SIZE + i]
                )
            );
        }

        op.init();
        assert!(!op.is_finished());

        let mut op = Modulation::new(mod_data, 1159);
        op.init();
        assert!(op.pack(&mut tx).is_err());

        let mut op = Modulation::new(vec![0.0; MOD_BUF_SIZE_MAX + 1], 1160);
        op.init();
        assert!(op.pack(&mut tx).is_err());
    }
}
