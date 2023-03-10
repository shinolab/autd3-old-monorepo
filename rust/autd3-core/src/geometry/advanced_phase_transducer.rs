/*
 * File: advanced_phase_transducer.rs
 * Project: geometry
 * Created Date: 31/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3_driver::{FPGA_CLK_FREQ, MAX_CYCLE};

use crate::error::AUTDInternalError;

use super::{Transducer, UnitQuaternion, Vector3};

pub struct AdvancedPhaseTransducer {
    idx: usize,
    pos: Vector3,
    rot: UnitQuaternion,
    cycle: u16,
    mod_delay: u16,
}

impl Transducer for AdvancedPhaseTransducer {
    fn new(idx: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            idx,
            pos,
            rot,
            cycle: 4096,
            mod_delay: 0,
        }
    }

    fn position(&self) -> &Vector3 {
        &self.pos
    }

    fn rotation(&self) -> &UnitQuaternion {
        &self.rot
    }

    fn idx(&self) -> usize {
        self.idx
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn set_mod_delay(&mut self, delay: u16) {
        self.mod_delay = delay;
    }

    fn frequency(&self) -> f64 {
        FPGA_CLK_FREQ as f64 / self.cycle as f64
    }
}

impl AdvancedPhaseTransducer {
    pub fn cycle(&self) -> u16 {
        self.cycle
    }

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
