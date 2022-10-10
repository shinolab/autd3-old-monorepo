/*
 * File: normal_transducer.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

use anyhow::{Ok, Result};

use autd3_driver::{Drive, FPGA_CLK_FREQ, MAX_CYCLE};

use crate::error::AUTDInternalError;

use super::{Transducer, Vector3};

pub struct NormalTransducer {
    id: usize,
    pos: Vector3,
    x_direction: Vector3,
    y_direction: Vector3,
    z_direction: Vector3,
    cycle: u16,
    mod_delay: u16,
}

impl Transducer for NormalTransducer {
    fn new(
        id: usize,
        pos: Vector3,
        x_direction: Vector3,
        y_direction: Vector3,
        z_direction: Vector3,
    ) -> Self {
        Self {
            id,
            pos,
            x_direction,
            y_direction,
            z_direction,
            cycle: 4096,
            mod_delay: 0,
        }
    }
    fn align_phase_at(&self, dist: f64, sound_speed: f64) -> f64 {
        let wavelength = sound_speed * 1e3 / self.frequency();
        dist / wavelength
    }

    fn position(&self) -> &Vector3 {
        &self.pos
    }

    fn id(&self) -> usize {
        self.id
    }

    fn x_direction(&self) -> &Vector3 {
        &self.x_direction
    }

    fn y_direction(&self) -> &Vector3 {
        &self.y_direction
    }

    fn z_direction(&self) -> &Vector3 {
        &self.z_direction
    }

    fn cycle(&self) -> u16 {
        self.cycle
    }

    fn frequency(&self) -> f64 {
        FPGA_CLK_FREQ as f64 / self.cycle as f64
    }

    fn pack_head(tx: &mut autd3_driver::TxDatagram) {
        autd3_driver::normal_head(tx);
    }

    fn pack_body(
        phase_sent: &mut bool,
        duty_sent: &mut bool,
        drives: &[Drive],
        tx: &mut autd3_driver::TxDatagram,
    ) -> anyhow::Result<()> {
        if !*phase_sent {
            autd3_driver::normal_phase_body(drives, tx)?;
            *phase_sent = true;
        } else {
            autd3_driver::normal_duty_body(drives, tx)?;
            *duty_sent = true;
        }
        Ok(())
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn set_mod_delay(&mut self, delay: u16) {
        self.mod_delay = delay;
    }

    fn wavelength(&self, sound_speed: f64) -> f64 {
        sound_speed * 1e3 / self.frequency()
    }

    fn wavenumber(&self, sound_speed: f64) -> f64 {
        2.0 * PI * self.frequency() / (sound_speed * 1e3)
    }

    fn gain_stm_max() -> usize {
        autd3_driver::GAIN_STM_NORMAL_BUF_SIZE_MAX
    }
}

impl NormalTransducer {
    pub fn set_cycle(&mut self, cycle: u16) -> Result<()> {
        if cycle > MAX_CYCLE {
            return Err(AUTDInternalError::CycleOutOfRange(cycle).into());
        }
        self.cycle = cycle;
        Ok(())
    }

    pub fn set_frequency(&mut self, freq: f64) -> Result<()> {
        let cycle = (FPGA_CLK_FREQ as f64 / freq).round() as u16;
        self.set_cycle(cycle)
    }
}
