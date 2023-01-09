/*
 * File: stm_gain.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use super::Operation;
use crate::{
    CPUControlFlags, Drive, DriverError, FPGAControlFlags, Mode, TxDatagram, GAIN_STM_BUF_SIZE_MAX,
    GAIN_STM_LEGACY_BUF_SIZE_MAX, GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN,
    GAIN_STM_SAMPLING_FREQ_DIV_MIN,
};

pub trait GainSTMOp {
    fn set_sampling_freq_div(&mut self, freq_div: u32);
    fn sampling_freq_div(&self) -> u32;
    fn set_start_idx(&mut self, idx: Option<u16>);
    fn start_idx(&self) -> Option<u16>;
    fn set_finish_idx(&mut self, idx: Option<u16>);
    fn finish_idx(&self) -> Option<u16>;
}

#[derive(Default)]
pub struct GainSTMLegacy {
    sent: usize,
    pub drives: Vec<Vec<Drive>>,
    pub freq_div: u32,
    pub mode: Mode,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
}

impl GainSTMOp for GainSTMLegacy {
    fn set_sampling_freq_div(&mut self, freq_div: u32) {
        self.freq_div = freq_div;
    }

    fn sampling_freq_div(&self) -> u32 {
        self.freq_div
    }

    fn set_start_idx(&mut self, idx: Option<u16>) {
        self.start_idx = idx;
    }

    fn start_idx(&self) -> Option<u16> {
        self.start_idx
    }

    fn set_finish_idx(&mut self, idx: Option<u16>) {
        self.finish_idx = idx;
    }

    fn finish_idx(&self) -> Option<u16> {
        self.finish_idx
    }
}

impl Operation for GainSTMLegacy {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_END);

        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::LEGACY_MODE, true);
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_MODE, true);
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_GAIN_MODE, true);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        if self.drives.len() > GAIN_STM_LEGACY_BUF_SIZE_MAX {
            return Err(DriverError::GainSTMLegacySizeOutOfRange(self.drives.len()).into());
        }

        let mut is_last_frame = false;

        if let Some(idx) = self.start_idx {
            if idx as usize >= self.drives.len() {
                return Err(DriverError::STMStartIndexOutOfRange.into());
            }
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_START_IDX, true);
        } else {
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_START_IDX, false);
        }
        if let Some(idx) = self.finish_idx {
            if idx as usize >= self.drives.len() {
                return Err(DriverError::STMFinishIndexOutOfRange.into());
            }
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_FINISH_IDX, true);
        } else {
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_FINISH_IDX, false);
        }

        if self.sent == 0 {
            if self.freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::GainSTMLegacyFreqDivOutOfRange(self.freq_div).into());
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                d.gain_stm_initial_mut().set_freq_div(self.freq_div);
                d.gain_stm_initial_mut().set_mode(self.mode);
                d.gain_stm_initial_mut().set_cycle(self.drives.len());
                d.gain_stm_initial_mut()
                    .set_start_idx(self.start_idx.unwrap_or(0));
                d.gain_stm_initial_mut()
                    .set_finish_idx(self.finish_idx.unwrap_or(0));
            });
            self.sent += 1;
        } else {
            match self.mode {
                Mode::PhaseDutyFull => {
                    is_last_frame = self.sent + 1 >= self.drives.len() + 1;
                    tx.legacy_drives_mut()
                        .iter_mut()
                        .zip(&self.drives[self.sent - 1])
                        .for_each(|(d, s)| d.set(s));
                    self.sent += 1;
                }
                Mode::PhaseFull => {
                    is_last_frame = self.sent + 2 >= self.drives.len() + 1;
                    tx.legacy_phase_full_mut()
                        .iter_mut()
                        .zip(&self.drives[self.sent - 1])
                        .for_each(|(d, s)| d.set(0, s));
                    self.sent += 1;
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_full_mut()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(1, s));
                        self.sent += 1;
                    }
                }
                Mode::PhaseHalf => {
                    is_last_frame = self.sent + 4 >= self.drives.len() + 1;
                    tx.legacy_phase_half_mut()
                        .iter_mut()
                        .zip(&self.drives[self.sent - 1])
                        .for_each(|(d, s)| d.set(0, s));
                    self.sent += 1;
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(1, s));
                        self.sent += 1;
                    }
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(2, s));
                        self.sent += 1;
                    }
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(3, s));
                        self.sent += 1;
                    }
                }
            }
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        if is_last_frame {
            tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
        }

        tx.num_bodies = tx.num_devices();

        Ok(())
    }

    fn init(&mut self) {
        self.sent = 0;
    }

    fn is_finished(&self) -> bool {
        self.sent > self.drives.len()
    }
}

#[derive(Default)]
pub struct GainSTMNormal {
    sent: usize,
    next_duty: bool,
    pub drives: Vec<Vec<Drive>>,
    pub cycles: Vec<u16>,
    pub freq_div: u32,
    pub mode: Mode,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
}

impl GainSTMNormal {
    fn pack_phase(&self, tx: &mut TxDatagram) -> Result<()> {
        if self.drives.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(DriverError::GainSTMSizeOutOfRange(self.drives.len()).into());
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

        if let Some(idx) = self.start_idx {
            if idx as usize >= self.drives.len() {
                return Err(DriverError::STMStartIndexOutOfRange.into());
            }
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_START_IDX, true);
        } else {
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_START_IDX, false);
        }
        if let Some(idx) = self.finish_idx {
            if idx as usize >= self.drives.len() {
                return Err(DriverError::STMFinishIndexOutOfRange.into());
            }
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_FINISH_IDX, true);
        } else {
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_FINISH_IDX, false);
        }

        if self.sent == 0 {
            if self.freq_div < GAIN_STM_SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::GainSTMFreqDivOutOfRange(self.freq_div).into());
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                d.gain_stm_initial_mut().set_freq_div(self.freq_div);
                d.gain_stm_initial_mut().set_mode(self.mode);
                d.gain_stm_initial_mut().set_cycle(self.drives.len());
                d.gain_stm_initial_mut()
                    .set_start_idx(self.start_idx.unwrap_or(0));
                d.gain_stm_initial_mut()
                    .set_finish_idx(self.finish_idx.unwrap_or(0));
            });
        } else {
            tx.phases_mut()
                .iter_mut()
                .zip(self.drives[self.sent - 1].iter())
                .zip(self.cycles.iter())
                .for_each(|((d, s), &c)| d.set(s, c));
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        if self.sent + 1 == self.drives.len() + 1 {
            tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
        }

        tx.num_bodies = tx.num_devices();

        Ok(())
    }

    fn pack_duty(&self, tx: &mut TxDatagram) -> Result<()> {
        if self.drives.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(DriverError::GainSTMSizeOutOfRange(self.drives.len()).into());
        }

        tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

        tx.duties_mut()
            .iter_mut()
            .zip(self.drives[self.sent - 1].iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        if self.sent + 1 == self.drives.len() + 1 {
            tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
        }

        tx.num_bodies = tx.num_devices();

        Ok(())
    }
}

impl GainSTMOp for GainSTMNormal {
    fn set_sampling_freq_div(&mut self, freq_div: u32) {
        self.freq_div = freq_div;
    }

    fn sampling_freq_div(&self) -> u32 {
        self.freq_div
    }

    fn set_start_idx(&mut self, idx: Option<u16>) {
        self.start_idx = idx;
    }

    fn start_idx(&self) -> Option<u16> {
        self.start_idx
    }

    fn set_finish_idx(&mut self, idx: Option<u16>) {
        self.finish_idx = idx;
    }

    fn finish_idx(&self) -> Option<u16> {
        self.finish_idx
    }
}

impl Operation for GainSTMNormal {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_END);

        tx.header_mut()
            .fpga_flag
            .remove(FPGAControlFlags::LEGACY_MODE);
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_MODE, true);
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_GAIN_MODE, true);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        match self.mode {
            Mode::PhaseDutyFull => {
                if self.next_duty {
                    self.pack_duty(tx)?;
                    self.sent += 1;
                } else {
                    self.pack_phase(tx)?;
                }
                self.next_duty = !self.next_duty;
            }
            Mode::PhaseFull => {
                self.pack_phase(tx)?;
                self.sent += 1;
            }
            Mode::PhaseHalf => return Err(DriverError::PhaseHalfNotSupported.into()),
        }

        Ok(())
    }

    fn init(&mut self) {
        self.sent = 0;
        self.next_duty = false;
    }

    fn is_finished(&self) -> bool {
        self.sent > self.drives.len()
    }
}
