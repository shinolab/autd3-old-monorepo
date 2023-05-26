/*
 * File: dynamic_transducer.rs
 * Project: geometry
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::core::{error::AUTDInternalError, float, FPGA_CLK_FREQ, MAX_CYCLE};

use super::{Matrix4, Transducer, UnitQuaternion, Vector3, Vector4};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(u8)]
pub enum TransMode {
    #[default]
    Legacy,
    Advanced,
    AdvancedPhase,
}

pub struct DynamicTransducer {
    idx: usize,
    pos: Vector3,
    rot: UnitQuaternion,
    cycle: u16,
    mod_delay: u16,
}

impl Transducer for DynamicTransducer {
    type Gain = autd3::core::operation::GainLegacy;
    type GainSTM = autd3::core::operation::GainSTMLegacy;
    type Sync = autd3::core::operation::SyncLegacy;

    fn new(idx: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            idx,
            pos,
            rot,
            cycle: 4096,
            mod_delay: 0,
        }
    }

    fn affine(&mut self, t: Vector3, r: UnitQuaternion) {
        let rot_mat: Matrix4 = From::from(r);
        let trans_mat = rot_mat.append_translation(&t);
        let homo = Vector4::new(self.pos[0], self.pos[1], self.pos[2], 1.0);
        let new_pos = trans_mat * homo;
        self.pos = Vector3::new(new_pos[0], new_pos[1], new_pos[2]);
        self.rot = r * self.rot;
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

    fn frequency(&self) -> float {
        FPGA_CLK_FREQ as float / self.cycle as float
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn set_mod_delay(&mut self, delay: u16) {
        self.mod_delay = delay;
    }

    fn cycle(&self) -> u16 {
        self.cycle
    }
}

impl DynamicTransducer {
    pub fn set_cycle(&mut self, cycle: u16) -> Result<(), AUTDInternalError> {
        if cycle > MAX_CYCLE {
            return Err(AUTDInternalError::CycleOutOfRange(cycle));
        }
        self.cycle = cycle;
        Ok(())
    }

    pub fn set_frequency(&mut self, freq: float) -> Result<(), AUTDInternalError> {
        let cycle = (FPGA_CLK_FREQ as float / freq).round() as u16;
        self.set_cycle(cycle)
    }
}
