/*
 * File: gain.rs
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
use crate::{CPUControlFlags, Drive, DriverError, FPGAControlFlags, TxDatagram};

pub trait GainOp {
    fn drives(&self) -> &[Drive];
    fn set_drive(&mut self, idx: usize, amp: f64, phase: f64);
}

#[derive(Default)]
pub struct GainLegacy {
    sent: bool,
    pub drives: Vec<Drive>,
}

impl GainOp for GainLegacy {
    fn drives(&self) -> &[Drive] {
        &self.drives
    }

    fn set_drive(&mut self, idx: usize, amp: f64, phase: f64) {
        self.drives[idx].amp = amp;
        self.drives[idx].phase = phase;
    }
}

impl Operation for GainLegacy {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::LEGACY_MODE, true);
        tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

        tx.num_bodies = 0;

        if self.sent {
            return Ok(());
        }

        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            }
            .into());
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        tx.legacy_drives_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .for_each(|(d, s)| d.set(s));

        tx.num_bodies = tx.num_devices();

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
pub struct GainNormal {
    pub phase_sent: bool,
    pub duty_sent: bool,
    pub drives: Vec<Drive>,
    pub cycles: Vec<u16>,
}

impl GainNormal {
    fn pack_duty(&self, tx: &mut TxDatagram) -> Result<()> {
        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            }
            .into());
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

        tx.duties_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        tx.num_bodies = tx.num_devices();

        Ok(())
    }

    fn pack_phase(&self, tx: &mut TxDatagram) -> Result<()> {
        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            }
            .into());
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

        tx.phases_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        tx.num_bodies = tx.num_devices();

        Ok(())
    }
}

impl GainOp for GainNormal {
    fn drives(&self) -> &[Drive] {
        &self.drives
    }

    fn set_drive(&mut self, idx: usize, amp: f64, phase: f64) {
        self.drives[idx].amp = amp;
        self.drives[idx].phase = phase;
    }
}

impl Operation for GainNormal {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

        tx.header_mut()
            .fpga_flag
            .remove(FPGAControlFlags::LEGACY_MODE);
        tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        if !self.phase_sent {
            self.pack_phase(tx)?;
            self.phase_sent = true;
            return Ok(());
        }

        self.pack_duty(tx)?;
        self.duty_sent = true;

        Ok(())
    }

    fn init(&mut self) {
        self.phase_sent = false;
        self.duty_sent = false;
    }

    fn is_finished(&self) -> bool {
        self.phase_sent && self.duty_sent
    }
}

#[derive(Default)]
pub struct GainNormalPhase {
    sent: bool,
    pub drives: Vec<Drive>,
    pub cycles: Vec<u16>,
}

impl GainOp for GainNormalPhase {
    fn drives(&self) -> &[Drive] {
        &self.drives
    }

    fn set_drive(&mut self, idx: usize, amp: f64, phase: f64) {
        self.drives[idx].amp = amp;
        self.drives[idx].phase = phase;
    }
}

impl Operation for GainNormalPhase {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

        tx.header_mut()
            .fpga_flag
            .remove(FPGAControlFlags::LEGACY_MODE);
        tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            }
            .into());
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

        tx.phases_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        tx.num_bodies = tx.num_devices();

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
