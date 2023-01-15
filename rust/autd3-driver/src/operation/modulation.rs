/*
 * File: modulation.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{
    CPUControlFlags, DriverError, TxDatagram, MOD_BUF_SIZE_MAX, MOD_HEADER_INITIAL_DATA_SIZE,
    MOD_HEADER_SUBSEQUENT_DATA_SIZE, MOD_SAMPLING_FREQ_DIV_MIN,
};
use anyhow::Result;

pub struct Modulation {
    mod_data: Vec<u8>,
    sent: usize,
    freq_div: u32,
}

impl Modulation {
    pub fn new(mod_data: Vec<u8>, freq_div: u32) -> Self {
        Self {
            mod_data,
            sent: 0,
            freq_div,
        }
    }
}

impl Operation for Modulation {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        if self.mod_data.len() > MOD_BUF_SIZE_MAX {
            return Err(DriverError::ModulationSizeOutOfRange(self.mod_data.len()).into());
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
                return Err(DriverError::ModFreqDivOutOfRange(self.freq_div).into());
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::MOD_BEGIN, true);
            tx.header_mut().mod_initial_mut().freq_div = self.freq_div;
            tx.header_mut().mod_initial_mut().data[0..mod_size]
                .copy_from_slice(&self.mod_data[self.sent..]);
        } else {
            tx.header_mut().mod_subsequent_mut().data[0..mod_size]
                .copy_from_slice(&self.mod_data[self.sent..]);
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
