/*
 * File: dynamic_transducer.rs
 * Project: geometry
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    defined::{float, Drive},
    error::AUTDInternalError,
    fpga::{FPGA_CLK_FREQ, MAX_CYCLE},
    geometry::{Device, Geometry},
    operation::{stm::gain::GainSTMOpDelegate, GainOpDelegate, GainSTMMode},
};

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
    local_idx: usize,
    pos: Vector3,
    rot: UnitQuaternion,
    cycle: u16,
    mod_delay: u16,
    amp_filter: float,
    phase_filter: float,
}

pub struct GainOpDynamic {}
impl GainOpDelegate<DynamicTransducer> for GainOpDynamic {
    fn init(_: &Geometry<DynamicTransducer>) -> Result<HashMap<usize, usize>, AUTDInternalError> {
        unreachable!()
    }

    fn pack(
        _: &HashMap<usize, Vec<Drive>>,
        _: &HashMap<usize, usize>,
        _: &Device<DynamicTransducer>,
        _: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        unreachable!()
    }
}

pub struct GainSTMOpDynamic {}
impl GainSTMOpDelegate<DynamicTransducer> for GainSTMOpDynamic {
    fn init(
        _: usize,
        _: GainSTMMode,
        _: &Geometry<DynamicTransducer>,
    ) -> Result<HashMap<usize, usize>, AUTDInternalError> {
        unreachable!()
    }

    fn pack(
        _: &[HashMap<usize, Vec<Drive>>],
        _: &HashMap<usize, usize>,
        _: &mut HashMap<usize, usize>,
        _: GainSTMMode,
        _: u32,
        _: Option<u16>,
        _: Option<u16>,
        _: &Device<DynamicTransducer>,
        _: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        unreachable!()
    }

    fn commit(
        _: &[HashMap<usize, Vec<Drive>>],
        _: &mut HashMap<usize, usize>,
        _: &HashMap<usize, usize>,
        _: GainSTMMode,
        _: &Device<DynamicTransducer>,
    ) {
        unreachable!()
    }

    fn required_size(_: &HashMap<usize, usize>, _: &Device<DynamicTransducer>) -> usize {
        unreachable!()
    }
}

impl Transducer for DynamicTransducer {
    type GainOp = GainOpDynamic;
    type GainSTMOp = GainSTMOpDynamic;

    fn new(local_idx: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            local_idx,
            pos,
            rot,
            cycle: 4096,
            mod_delay: 0,
            amp_filter: 0.,
            phase_filter: 0.,
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

    fn local_idx(&self) -> usize {
        self.local_idx
    }

    fn frequency(&self) -> float {
        FPGA_CLK_FREQ as float / self.cycle as float
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn amp_filter(&self) -> float {
        self.amp_filter
    }

    fn set_amp_filter(&mut self, value: float) {
        self.amp_filter = value;
    }

    fn phase_filter(&self) -> float {
        self.phase_filter
    }

    fn set_phase_filter(&mut self, value: float) {
        self.phase_filter = value;
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
