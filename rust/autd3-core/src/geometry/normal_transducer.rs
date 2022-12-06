/*
 * File: normal_transducer.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::{Ok, Result};

use autd3_driver::{FPGA_CLK_FREQ, MAX_CYCLE};

use crate::error::AUTDInternalError;

use super::{Transducer, UnitQuaternion, Vector3};

pub struct NormalTransducer {
    id: usize,
    pos: Vector3,
    rot: UnitQuaternion,
    cycle: u16,
    sound_speed: f64,
    attenuation: f64,
    mod_delay: u16,
}

impl Transducer for NormalTransducer {
    fn new(id: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            id,
            pos,
            rot,
            cycle: 4096,
            sound_speed: 340e3,
            attenuation: 0.,
            mod_delay: 0,
        }
    }

    fn position(&self) -> &Vector3 {
        &self.pos
    }

    fn rotation(&self) -> &UnitQuaternion {
        &self.rot
    }

    fn id(&self) -> usize {
        self.id
    }

    fn cycle(&self) -> u16 {
        self.cycle
    }

    fn frequency(&self) -> f64 {
        FPGA_CLK_FREQ as f64 / self.cycle as f64
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn set_mod_delay(&mut self, delay: u16) {
        self.mod_delay = delay;
    }

    fn sound_speed(&self) -> f64 {
        self.sound_speed
    }

    fn set_sound_speed(&mut self, value: f64) {
        self.sound_speed = value;
    }

    fn attenuation(&self) -> f64 {
        self.attenuation
    }

    fn set_attenuation(&mut self, value: f64) {
        self.attenuation = value;
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
