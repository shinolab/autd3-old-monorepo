/*
 * File: stm_gain.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/03/2023
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

#[derive(Default, Clone, Copy)]
pub struct GainSTMProps {
    pub freq_div: u32,
    pub mode: Mode,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
}

pub struct GainSTMLegacy {
    sent: usize,
    drives: Vec<Vec<Drive>>,
    props: GainSTMProps,
}

impl GainSTMLegacy {
    pub fn new(drives: Vec<Vec<Drive>>, props: GainSTMProps) -> Self {
        Self {
            sent: 0,
            drives,
            props,
        }
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

        if let Some(idx) = self.props.start_idx {
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
        if let Some(idx) = self.props.finish_idx {
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
            if self.props.freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN {
                return Err(
                    DriverError::GainSTMLegacyFreqDivOutOfRange(self.props.freq_div).into(),
                );
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                d.gain_stm_initial_mut().set_freq_div(self.props.freq_div);
                d.gain_stm_initial_mut().set_mode(self.props.mode);
                d.gain_stm_initial_mut().set_cycle(self.drives.len());
                d.gain_stm_initial_mut()
                    .set_start_idx(self.props.start_idx.unwrap_or(0));
                d.gain_stm_initial_mut()
                    .set_finish_idx(self.props.finish_idx.unwrap_or(0));
            });
            self.sent += 1;
        } else {
            match self.props.mode {
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
                    tx.legacy_phase_full_mut::<0>()
                        .iter_mut()
                        .zip(&self.drives[self.sent - 1])
                        .for_each(|(d, s)| d.set(s));
                    self.sent += 1;
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_full_mut::<1>()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(s));
                        self.sent += 1;
                    }
                }
                Mode::PhaseHalf => {
                    is_last_frame = self.sent + 4 >= self.drives.len() + 1;
                    tx.legacy_phase_half_mut::<0>()
                        .iter_mut()
                        .zip(&self.drives[self.sent - 1])
                        .for_each(|(d, s)| d.set(s));
                    self.sent += 1;
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut::<1>()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(s));
                        self.sent += 1;
                    }
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut::<2>()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(s));
                        self.sent += 1;
                    }
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut::<3>()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(s));
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
pub struct GainSTMAdvanced {
    sent: usize,
    next_duty: bool,
    drives: Vec<Vec<Drive>>,
    cycles: Vec<u16>,
    props: GainSTMProps,
}

impl GainSTMAdvanced {
    pub fn new(drives: Vec<Vec<Drive>>, cycles: Vec<u16>, props: GainSTMProps) -> Self {
        Self {
            sent: 0,
            next_duty: false,
            drives,
            cycles,
            props,
        }
    }

    fn pack_phase(&self, tx: &mut TxDatagram) -> Result<()> {
        if self.drives.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(DriverError::GainSTMSizeOutOfRange(self.drives.len()).into());
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

        if let Some(idx) = self.props.start_idx {
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
        if let Some(idx) = self.props.finish_idx {
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
            if self.props.freq_div < GAIN_STM_SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::GainSTMFreqDivOutOfRange(self.props.freq_div).into());
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                d.gain_stm_initial_mut().set_freq_div(self.props.freq_div);
                d.gain_stm_initial_mut().set_mode(self.props.mode);
                d.gain_stm_initial_mut().set_cycle(self.drives.len());
                d.gain_stm_initial_mut()
                    .set_start_idx(self.props.start_idx.unwrap_or(0));
                d.gain_stm_initial_mut()
                    .set_finish_idx(self.props.finish_idx.unwrap_or(0));
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

impl Operation for GainSTMAdvanced {
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

        match self.props.mode {
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
