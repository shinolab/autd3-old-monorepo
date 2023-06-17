/*
 * File: fpga_emulator.rs
 * Project: src
 * Created Date: 06/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::error::AUTDExtraError;

use autd3_core::geometry::Vector3;

use super::params::*;

use num_integer::Roots;

pub struct FPGAEmulator {
    controller_bram: Vec<u16>,
    modulator_bram: Vec<u16>,
    normal_op_bram: Vec<u16>,
    stm_op_bram: Vec<u16>,
    num_transducers: usize,
    tr_pos: Vec<u64>,
}

impl FPGAEmulator {
    pub(crate) fn new(num_transducers: usize) -> Self {
        Self {
            controller_bram: vec![0x0000; 1024],
            modulator_bram: vec![0x0000; 32768],
            normal_op_bram: vec![0x0000; 512],
            stm_op_bram: vec![0x0000; 524288],
            num_transducers,
            tr_pos: vec![
                0x00000000, 0x01960000, 0x032c0000, 0x04c30000, 0x06590000, 0x07ef0000, 0x09860000,
                0x0b1c0000, 0x0cb30000, 0x0e490000, 0x0fdf0000, 0x11760000, 0x130c0000, 0x14a30000,
                0x16390000, 0x17d00000, 0x19660000, 0x1afc0000, 0x00000196, 0x04c30196, 0x06590196,
                0x07ef0196, 0x09860196, 0x0b1c0196, 0x0cb30196, 0x0e490196, 0x0fdf0196, 0x11760196,
                0x130c0196, 0x14a30196, 0x16390196, 0x17d00196, 0x1afc0196, 0x0000032c, 0x0196032c,
                0x032c032c, 0x04c3032c, 0x0659032c, 0x07ef032c, 0x0986032c, 0x0b1c032c, 0x0cb3032c,
                0x0e49032c, 0x0fdf032c, 0x1176032c, 0x130c032c, 0x14a3032c, 0x1639032c, 0x17d0032c,
                0x1966032c, 0x1afc032c, 0x000004c3, 0x019604c3, 0x032c04c3, 0x04c304c3, 0x065904c3,
                0x07ef04c3, 0x098604c3, 0x0b1c04c3, 0x0cb304c3, 0x0e4904c3, 0x0fdf04c3, 0x117604c3,
                0x130c04c3, 0x14a304c3, 0x163904c3, 0x17d004c3, 0x196604c3, 0x1afc04c3, 0x00000659,
                0x01960659, 0x032c0659, 0x04c30659, 0x06590659, 0x07ef0659, 0x09860659, 0x0b1c0659,
                0x0cb30659, 0x0e490659, 0x0fdf0659, 0x11760659, 0x130c0659, 0x14a30659, 0x16390659,
                0x17d00659, 0x19660659, 0x1afc0659, 0x000007ef, 0x019607ef, 0x032c07ef, 0x04c307ef,
                0x065907ef, 0x07ef07ef, 0x098607ef, 0x0b1c07ef, 0x0cb307ef, 0x0e4907ef, 0x0fdf07ef,
                0x117607ef, 0x130c07ef, 0x14a307ef, 0x163907ef, 0x17d007ef, 0x196607ef, 0x1afc07ef,
                0x00000986, 0x01960986, 0x032c0986, 0x04c30986, 0x06590986, 0x07ef0986, 0x09860986,
                0x0b1c0986, 0x0cb30986, 0x0e490986, 0x0fdf0986, 0x11760986, 0x130c0986, 0x14a30986,
                0x16390986, 0x17d00986, 0x19660986, 0x1afc0986, 0x00000b1c, 0x01960b1c, 0x032c0b1c,
                0x04c30b1c, 0x06590b1c, 0x07ef0b1c, 0x09860b1c, 0x0b1c0b1c, 0x0cb30b1c, 0x0e490b1c,
                0x0fdf0b1c, 0x11760b1c, 0x130c0b1c, 0x14a30b1c, 0x16390b1c, 0x17d00b1c, 0x19660b1c,
                0x1afc0b1c, 0x00000cb3, 0x01960cb3, 0x032c0cb3, 0x04c30cb3, 0x06590cb3, 0x07ef0cb3,
                0x09860cb3, 0x0b1c0cb3, 0x0cb30cb3, 0x0e490cb3, 0x0fdf0cb3, 0x11760cb3, 0x130c0cb3,
                0x14a30cb3, 0x16390cb3, 0x17d00cb3, 0x19660cb3, 0x1afc0cb3, 0x00000e49, 0x01960e49,
                0x032c0e49, 0x04c30e49, 0x06590e49, 0x07ef0e49, 0x09860e49, 0x0b1c0e49, 0x0cb30e49,
                0x0e490e49, 0x0fdf0e49, 0x11760e49, 0x130c0e49, 0x14a30e49, 0x16390e49, 0x17d00e49,
                0x19660e49, 0x1afc0e49, 0x00000fdf, 0x01960fdf, 0x032c0fdf, 0x04c30fdf, 0x06590fdf,
                0x07ef0fdf, 0x09860fdf, 0x0b1c0fdf, 0x0cb30fdf, 0x0e490fdf, 0x0fdf0fdf, 0x11760fdf,
                0x130c0fdf, 0x14a30fdf, 0x16390fdf, 0x17d00fdf, 0x19660fdf, 0x1afc0fdf, 0x00001176,
                0x01961176, 0x032c1176, 0x04c31176, 0x06591176, 0x07ef1176, 0x09861176, 0x0b1c1176,
                0x0cb31176, 0x0e491176, 0x0fdf1176, 0x11761176, 0x130c1176, 0x14a31176, 0x16391176,
                0x17d01176, 0x19661176, 0x1afc1176, 0x0000130c, 0x0196130c, 0x032c130c, 0x04c3130c,
                0x0659130c, 0x07ef130c, 0x0986130c, 0x0b1c130c, 0x0cb3130c, 0x0e49130c, 0x0fdf130c,
                0x1176130c, 0x130c130c, 0x14a3130c, 0x1639130c, 0x17d0130c, 0x1966130c, 0x1afc130c,
                0x000014a3, 0x019614a3, 0x032c14a3, 0x04c314a3, 0x065914a3, 0x07ef14a3, 0x098614a3,
                0x0b1c14a3, 0x0cb314a3, 0x0e4914a3, 0x0fdf14a3, 0x117614a3, 0x130c14a3, 0x14a314a3,
                0x163914a3, 0x17d014a3, 0x196614a3, 0x1afc14a3,
            ],
        }
    }

    pub(crate) fn init(&mut self) {
        self.controller_bram[ADDR_VERSION_NUM] =
            (ENABLED_FEATURES_BITS as u16) << 8 | VERSION_NUM_MAJOR as u16;
        self.controller_bram[ADDR_VERSION_NUM_MINOR] = VERSION_NUM_MINOR as u16;
    }

    pub(crate) fn read(&self, addr: u16) -> u16 {
        let select = (addr >> 14) & 0x0003;
        let addr = (addr & 0x3FFF) as usize;
        match select {
            BRAM_SELECT_CONTROLLER => self.controller_bram[addr],
            BRAM_SELECT_MOD => {
                let offset = self.controller_bram[ADDR_MOD_ADDR_OFFSET];
                let addr = (offset as usize) << 14 | addr;
                self.modulator_bram[addr]
            }
            BRAM_SELECT_NORMAL => self.normal_op_bram[addr],
            BRAM_SELECT_STM => {
                let offset = self.controller_bram[ADDR_STM_ADDR_OFFSET];
                let addr = (offset as usize) << 14 | addr;
                self.stm_op_bram[addr]
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn write(&mut self, addr: u16, data: u16) {
        let select = (addr >> 14) & 0x0003;
        let addr = (addr & 0x3FFF) as usize;
        match select {
            BRAM_SELECT_CONTROLLER => self.controller_bram[addr] = data,
            BRAM_SELECT_MOD => {
                let offset = self.controller_bram[ADDR_MOD_ADDR_OFFSET];
                let addr = (offset as usize) << 14 | addr;
                self.modulator_bram[addr] = data;
            }
            BRAM_SELECT_NORMAL => self.normal_op_bram[addr] = data,
            BRAM_SELECT_STM => {
                let offset = self.controller_bram[ADDR_STM_ADDR_OFFSET];
                let addr = (offset as usize) << 14 | addr;
                self.stm_op_bram[addr] = data
            }
            _ => unreachable!(),
        }
    }

    pub fn assert_thermal_sensor(&mut self) {
        self.controller_bram[ADDR_FPGA_INFO] |= 0x0001;
    }

    pub fn deassert_thermal_sensor(&mut self) {
        self.controller_bram[ADDR_FPGA_INFO] &= !0x0001;
    }

    pub fn is_legacy_mode(&self) -> bool {
        (self.controller_bram[ADDR_CTL_REG] & (1 << CTL_REG_LEGACY_MODE_BIT)) != 0
    }

    pub fn is_force_fan(&self) -> bool {
        (self.controller_bram[ADDR_CTL_REG] & (1 << CTL_REG_FORCE_FAN_BIT)) != 0
    }

    pub fn is_stm_mode(&self) -> bool {
        (self.controller_bram[ADDR_CTL_REG] & (1 << CTL_REG_OP_MODE_BIT)) != 0
    }

    pub fn is_stm_gain_mode(&self) -> bool {
        (self.controller_bram[ADDR_CTL_REG] & (1 << CTL_REG_STM_GAIN_MODE_BIT)) != 0
    }

    pub fn silencer_step(&self) -> u16 {
        self.controller_bram[ADDR_SILENT_STEP]
    }

    pub fn cycles(&self) -> Vec<u16> {
        self.controller_bram[ADDR_CYCLE_BASE..]
            .iter()
            .take(self.num_transducers)
            .copied()
            .collect()
    }

    pub fn mod_delays(&self) -> Vec<u16> {
        self.controller_bram[ADDR_MOD_DELAY_BASE..]
            .iter()
            .take(self.num_transducers)
            .copied()
            .collect()
    }

    pub fn stm_frequency_division(&self) -> u32 {
        ((self.controller_bram[ADDR_STM_FREQ_DIV_1] as u32) << 16) & 0xFFFF0000
            | self.controller_bram[ADDR_STM_FREQ_DIV_0] as u32 & 0x0000FFFF
    }

    pub fn stm_cycle(&self) -> usize {
        self.controller_bram[ADDR_STM_CYCLE] as usize + 1
    }

    pub fn sound_speed(&self) -> u32 {
        ((self.controller_bram[ADDR_SOUND_SPEED_1] as u32) << 16) & 0xFFFF0000
            | self.controller_bram[ADDR_SOUND_SPEED_0] as u32 & 0x0000FFFF
    }

    pub fn stm_start_idx(&self) -> Option<u16> {
        if self.controller_bram[ADDR_CTL_REG] & (1 << CTL_FLAG_USE_STM_START_IDX_BIT) != 0 {
            Some(self.controller_bram[ADDR_STM_START_IDX])
        } else {
            None
        }
    }

    pub fn stm_finish_idx(&self) -> Option<u16> {
        if self.controller_bram[ADDR_CTL_REG] & (1 << CTL_FLAG_USE_STM_FINISH_IDX_BIT) != 0 {
            Some(self.controller_bram[ADDR_STM_FINISH_IDX])
        } else {
            None
        }
    }

    pub fn modulation_frequency_division(&self) -> u32 {
        ((self.controller_bram[ADDR_MOD_FREQ_DIV_1] as u32) << 16) & 0xFFFF0000
            | self.controller_bram[ADDR_MOD_FREQ_DIV_0] as u32 & 0x0000FFFF
    }

    pub fn modulation_cycle(&self) -> usize {
        self.controller_bram[ADDR_MOD_CYCLE] as usize + 1
    }

    pub fn modulation_at(&self, idx: usize) -> u8 {
        let m = if idx % 2 == 0 {
            self.modulator_bram[idx >> 1] & 0xFF
        } else {
            self.modulator_bram[idx >> 1] >> 8
        };
        m as u8
    }

    pub fn modulation(&self) -> Vec<u8> {
        let cycle = self.modulation_cycle();
        let mut m = Vec::with_capacity(cycle);
        (0..cycle >> 1).for_each(|i| {
            let b = self.modulator_bram[i];
            m.push((b & 0x00FF) as u8);
            m.push(((b >> 8) & 0x00FF) as u8);
        });
        if cycle % 2 != 0 {
            let b = self.modulator_bram[(cycle + 1) >> 1];
            m.push((b & 0x00FF) as u8);
        }
        m
    }

    pub fn is_outputting(&self) -> bool {
        if self.modulation().iter().all(|&m| m == 0) {
            return false;
        }
        if !self.is_stm_mode() {
            let drives = self.duties_and_phases(0);
            return if self.is_legacy_mode() {
                drives.iter().any(|&d| d.0 > 8)
            } else {
                drives.iter().any(|&d| d.0 != 0)
            };
        }
        true
    }

    pub fn duties_and_phases(&self, idx: usize) -> Vec<(u16, u16)> {
        if self.is_stm_mode() {
            if self.is_stm_gain_mode() {
                if self.is_legacy_mode() {
                    self.gain_stm_legacy_duties_and_phases(idx)
                } else {
                    self.gain_stm_advanced_duties_and_phases(idx)
                }
            } else {
                self.focus_stm_duties_and_phases(idx)
            }
        } else if self.is_legacy_mode() {
            self.legacy_duties_and_phases()
        } else {
            self.advanced_duties_and_phases()
        }
    }

    fn legacy_duties_and_phases(&self) -> Vec<(u16, u16)> {
        self.normal_op_bram
            .iter()
            .step_by(2)
            .take(self.num_transducers)
            .map(|d| {
                let duty = (d >> 8) & 0xFF;
                let duty = ((duty << 3) | 0x07) + 1;
                let phase = d & 0xFF;
                let phase = phase << 4;
                (duty, phase)
            })
            .collect()
    }

    fn advanced_duties_and_phases(&self) -> Vec<(u16, u16)> {
        self.normal_op_bram
            .chunks(2)
            .take(self.num_transducers)
            .map(|x| (x[1], x[0]))
            .collect()
    }

    fn gain_stm_advanced_duties_and_phases(&self, idx: usize) -> Vec<(u16, u16)> {
        self.stm_op_bram
            .chunks(2)
            .skip(256 * idx)
            .take(self.num_transducers)
            .map(|x| (x[1], x[0]))
            .collect()
    }

    fn gain_stm_legacy_duties_and_phases(&self, idx: usize) -> Vec<(u16, u16)> {
        self.stm_op_bram
            .iter()
            .skip(256 * idx)
            .take(self.num_transducers)
            .map(|&d| {
                let duty = (d >> 8) & 0xFF;
                let duty = ((duty << 3) | 0x07) + 1;
                let phase = d & 0xFF;
                let phase = phase << 4;
                (duty, phase)
            })
            .collect()
    }

    pub fn focus_stm_duties_and_phases(&self, idx: usize) -> Vec<(u16, u16)> {
        let ultrasound_cycles = self.cycles();
        let sound_speed = self.sound_speed() as u64;
        let duty_shift = (self.stm_op_bram[8 * idx + 3] >> 6 & 0x000F) + 1;

        let mut x = (self.stm_op_bram[8 * idx + 1] as u32) << 16 & 0x30000;
        x |= self.stm_op_bram[8 * idx] as u32;
        let x = if (x & 0x20000) != 0 {
            -131072 + (x & 0x1FFFF) as i32
        } else {
            x as i32
        };
        let mut y = (self.stm_op_bram[8 * idx + 2] as u32) << 14 & 0x3C000;
        y |= self.stm_op_bram[8 * idx + 1] as u32 >> 2;
        let y = if (y & 0x20000) != 0 {
            -131072 + (y & 0x1FFFF) as i32
        } else {
            y as i32
        };
        let mut z = (self.stm_op_bram[8 * idx + 3] as u32) << 12 & 0x3F000;
        z |= self.stm_op_bram[8 * idx + 2] as u32 >> 4;
        let z = if (z & 0x20000) != 0 {
            -131072 + (z & 0x1FFFF) as i32
        } else {
            z as i32
        };
        self.tr_pos
            .iter()
            .zip(ultrasound_cycles.iter())
            .map(|(&tr, &cycle)| {
                let tr_z = (tr >> 32 & 0xFFFF) as i32;
                let tr_x = (tr >> 16 & 0xFFFF) as i32;
                let tr_y = (tr & 0xFFFF) as i32;
                let d2 =
                    (x - tr_x) * (x - tr_x) + (y - tr_y) * (y - tr_y) + (z - tr_z) * (z - tr_z);
                let dist = d2.sqrt() as u64;
                let q = (dist << 22) / sound_speed;
                let p = q % cycle as u64;
                (cycle >> duty_shift, p as u16)
            })
            .collect()
    }

    #[deprecated(since = "11.3.0")]
    #[allow(deprecated)]
    pub fn drives(&self, idx: usize) -> (Vec<u16>, Vec<u16>) {
        if self.is_stm_mode() {
            if self.is_stm_gain_mode() {
                if self.is_legacy_mode() {
                    self.gain_stm_legacy_drives(idx)
                } else {
                    self.gain_stm_advanced_drives(idx)
                }
            } else {
                self.focus_stm_drives(idx)
            }
        } else if self.is_legacy_mode() {
            self.legacy_drive()
        } else {
            self.advanced_drive()
        }
    }

    #[deprecated(since = "11.3.0")]
    fn legacy_drive(&self) -> (Vec<u16>, Vec<u16>) {
        (
            self.normal_op_bram
                .iter()
                .step_by(2)
                .take(self.num_transducers)
                .map(|d| {
                    let duty = (d >> 8) & 0xFF;
                    ((duty << 3) | 0x07) + 1
                })
                .collect(),
            self.normal_op_bram
                .iter()
                .step_by(2)
                .take(self.num_transducers)
                .map(|d| {
                    let phase = d & 0xFF;
                    phase << 4
                })
                .collect(),
        )
    }

    #[deprecated(since = "11.3.0")]
    fn advanced_drive(&self) -> (Vec<u16>, Vec<u16>) {
        (
            self.normal_op_bram
                .iter()
                .skip(1)
                .step_by(2)
                .take(self.num_transducers)
                .copied()
                .collect(),
            self.normal_op_bram
                .iter()
                .step_by(2)
                .take(self.num_transducers)
                .copied()
                .collect(),
        )
    }

    #[deprecated(since = "11.3.0")]
    fn gain_stm_advanced_drives(&self, idx: usize) -> (Vec<u16>, Vec<u16>) {
        (
            self.stm_op_bram
                .iter()
                .skip(512 * idx + 1)
                .step_by(2)
                .take(self.num_transducers)
                .copied()
                .collect(),
            self.stm_op_bram
                .iter()
                .skip(512 * idx)
                .step_by(2)
                .take(self.num_transducers)
                .copied()
                .collect(),
        )
    }

    #[deprecated(since = "11.3.0")]
    fn gain_stm_legacy_drives(&self, idx: usize) -> (Vec<u16>, Vec<u16>) {
        (
            self.stm_op_bram
                .iter()
                .skip(256 * idx)
                .take(self.num_transducers)
                .map(|&d| {
                    let duty = (d >> 8) & 0xFF;
                    ((duty << 3) | 0x07) + 1
                })
                .collect(),
            self.stm_op_bram
                .iter()
                .skip(256 * idx)
                .take(self.num_transducers)
                .map(|d| {
                    let phase = d & 0xFF;
                    phase << 4
                })
                .collect(),
        )
    }

    #[deprecated(since = "11.3.0")]
    pub fn focus_stm_drives(&self, idx: usize) -> (Vec<u16>, Vec<u16>) {
        let ultrasound_cycles = self.cycles();
        let sound_speed = self.sound_speed() as u64;
        let duty_shift = (self.stm_op_bram[8 * idx + 3] >> 6 & 0x000F) + 1;
        (
            ultrasound_cycles
                .iter()
                .map(|&cycle| cycle >> duty_shift)
                .collect(),
            {
                let mut x = (self.stm_op_bram[8 * idx + 1] as u32) << 16 & 0x30000;
                x |= self.stm_op_bram[8 * idx] as u32;
                let x = if (x & 0x20000) != 0 {
                    -131072 + (x & 0x1FFFF) as i32
                } else {
                    x as i32
                };
                let mut y = (self.stm_op_bram[8 * idx + 2] as u32) << 14 & 0x3C000;
                y |= self.stm_op_bram[8 * idx + 1] as u32 >> 2;
                let y = if (y & 0x20000) != 0 {
                    -131072 + (y & 0x1FFFF) as i32
                } else {
                    y as i32
                };
                let mut z = (self.stm_op_bram[8 * idx + 3] as u32) << 12 & 0x3F000;
                z |= self.stm_op_bram[8 * idx + 2] as u32 >> 4;
                let z = if (z & 0x20000) != 0 {
                    -131072 + (z & 0x1FFFF) as i32
                } else {
                    z as i32
                };
                self.tr_pos
                    .iter()
                    .zip(ultrasound_cycles.iter())
                    .map(|(&tr, &cycle)| {
                        let tr_z = (tr >> 32 & 0xFFFF) as i32;
                        let tr_x = (tr >> 16 & 0xFFFF) as i32;
                        let tr_y = (tr & 0xFFFF) as i32;
                        let d2 = (x - tr_x) * (x - tr_x)
                            + (y - tr_y) * (y - tr_y)
                            + (z - tr_z) * (z - tr_z);
                        let dist = d2.sqrt() as u64;
                        let q = (dist << 22) / sound_speed;
                        let p = q % cycle as u64;
                        p as u16
                    })
                    .collect()
            },
        )
    }

    pub fn configure_local_trans_pos(
        &mut self,
        local_trans_pos: Vec<Vector3>,
    ) -> Result<(), AUTDExtraError> {
        if local_trans_pos.len() != self.num_transducers {}

        self.tr_pos = local_trans_pos
            .iter()
            .map(|tr| {
                let x = (tr.x * TRANS_SIZE_FIXED_POINT_UNIT).round() as u16;
                let y = (tr.y * TRANS_SIZE_FIXED_POINT_UNIT).round() as u16;
                let z = (tr.z * TRANS_SIZE_FIXED_POINT_UNIT).round() as u16;
                ((z as u64) << 32) | ((x as u64) << 16) | (y as u64)
            })
            .collect();

        Ok(())
    }
}
