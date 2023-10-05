/*
 * File: gain.rs
 * Project: stm
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, fmt::Display};

use crate::{
    datagram::{Gain, GainFilter},
    defined::Drive,
    error::AUTDInternalError,
    fpga::{
        AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive, FPGA_SUB_CLK_FREQ_DIV,
        GAIN_STM_BUF_SIZE_MAX, GAIN_STM_LEGACY_BUF_SIZE_MAX, SAMPLING_FREQ_DIV_MIN,
    },
    geometry::{
        AdvancedPhaseTransducer, AdvancedTransducer, Device, Geometry, LegacyTransducer, Transducer,
    },
    operation::{Operation, TypeTag},
};

use std::fmt;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct GainSTMControlFlags : u8 {
        const NONE            = 0;
        const LEGACY          = 1 << 0;
        const DUTY            = 1 << 1;
        const STM_BEGIN       = 1 << 2;
        const STM_END         = 1 << 3;
        const USE_START_IDX   = 1 << 4;
        const USE_FINISH_IDX  = 1 << 5;
        const _RESERVED_0     = 1 << 6;
        const _RESERVED_1     = 1 << 7;
    }
}

impl fmt::Display for GainSTMControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(GainSTMControlFlags::LEGACY) {
            flags.push("LEGACY")
        }
        if self.contains(GainSTMControlFlags::DUTY) {
            flags.push("DUTY")
        }
        if self.contains(GainSTMControlFlags::STM_BEGIN) {
            flags.push("STM_BEGIN")
        }
        if self.contains(GainSTMControlFlags::STM_END) {
            flags.push("STM_END")
        }
        if self.contains(GainSTMControlFlags::USE_START_IDX) {
            flags.push("USE_START_IDX")
        }
        if self.contains(GainSTMControlFlags::USE_FINISH_IDX) {
            flags.push("USE_FINISH_IDX")
        }
        if self.is_empty() {
            flags.push("NONE")
        }
        write!(
            f,
            "{}",
            flags
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GainSTMMode {
    #[default]
    PhaseDutyFull = 0,
    PhaseFull = 1,
    PhaseHalf = 2,
}

impl From<u16> for GainSTMMode {
    fn from(v: u16) -> Self {
        match v {
            0 => GainSTMMode::PhaseDutyFull,
            1 => GainSTMMode::PhaseFull,
            2 => GainSTMMode::PhaseHalf,
            _ => unreachable!(),
        }
    }
}

impl Display for GainSTMMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GainSTMMode::PhaseDutyFull => write!(f, "PhaseDutyFull"),
            GainSTMMode::PhaseFull => write!(f, "PhaseFull"),
            GainSTMMode::PhaseHalf => write!(f, "PhaseHalf"),
        }
    }
}

#[repr(C)]
pub struct LegacyPhaseFull<const N: usize> {
    phase_0: u8,
    phase_1: u8,
}

impl LegacyPhaseFull<0> {
    pub fn set(&mut self, d: &Drive) {
        self.phase_0 = LegacyDrive::to_phase(d);
    }
}

impl LegacyPhaseFull<1> {
    pub fn set(&mut self, d: &Drive) {
        self.phase_1 = LegacyDrive::to_phase(d);
    }
}

#[repr(C)]
pub struct LegacyPhaseHalf<const N: usize> {
    phase_01: u8,
    phase_23: u8,
}

impl LegacyPhaseHalf<0> {
    pub fn set(&mut self, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        self.phase_01 = (self.phase_01 & 0xF0) | ((phase >> 4) & 0x0F);
    }
}

impl LegacyPhaseHalf<1> {
    pub fn set(&mut self, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        self.phase_01 = (self.phase_01 & 0x0F) | (phase & 0xF0);
    }
}

impl LegacyPhaseHalf<2> {
    pub fn set(&mut self, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        self.phase_23 = (self.phase_23 & 0xF0) | ((phase >> 4) & 0x0F);
    }
}

impl LegacyPhaseHalf<3> {
    pub fn set(&mut self, d: &Drive) {
        let phase = LegacyDrive::to_phase(d);
        self.phase_23 = (self.phase_23 & 0x0F) | (phase & 0xF0);
    }
}

pub struct GainSTMOp<T: Transducer, G: Gain<T>> {
    gains: Vec<G>,
    drives: Vec<HashMap<usize, Vec<Drive>>>,
    remains: HashMap<usize, usize>,
    sent: HashMap<usize, usize>,
    mode: GainSTMMode,
    freq_div: u32,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Transducer, G: Gain<T>> GainSTMOp<T, G> {
    pub fn new(
        gains: Vec<G>,
        mode: GainSTMMode,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
    ) -> Self {
        Self {
            gains,
            drives: Default::default(),
            remains: Default::default(),
            sent: Default::default(),
            mode,
            freq_div,
            start_idx,
            finish_idx,
            phantom: Default::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pack_legacy(
        drives: &Vec<HashMap<usize, Vec<Drive>>>,
        remains: &HashMap<usize, usize>,
        sent_map: &mut HashMap<usize, usize>,
        mode: GainSTMMode,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        assert!(remains[&device.idx()] > 0);

        tx[0] = TypeTag::GainSTM as u8;

        let sent = sent_map[&device.idx()];
        let mut offset =
            std::mem::size_of::<TypeTag>() + std::mem::size_of::<GainSTMControlFlags>();
        if sent == 0 {
            offset += std::mem::size_of::<GainSTMMode>()
            + std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>(); // finish idx
        }
        assert!(tx.len() >= offset + device.num_transducers() * std::mem::size_of::<LegacyDrive>());

        let mut f = GainSTMControlFlags::LEGACY;
        f.set(GainSTMControlFlags::STM_BEGIN, sent == 0);

        if sent == 0 {
            let mode = mode as u16;
            tx[2] = (mode & 0xFF) as u8;
            tx[3] = (mode >> 8) as u8;

            let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;
            tx[4] = (freq_div & 0xFF) as u8;
            tx[5] = ((freq_div >> 8) & 0xFF) as u8;
            tx[6] = ((freq_div >> 16) & 0xFF) as u8;
            tx[7] = ((freq_div >> 24) & 0xFF) as u8;

            f.set(GainSTMControlFlags::USE_START_IDX, start_idx.is_some());
            let start_idx = start_idx.unwrap_or(0);
            tx[8] = (start_idx & 0xFF) as u8;
            tx[9] = (start_idx >> 8) as u8;

            f.set(GainSTMControlFlags::USE_FINISH_IDX, finish_idx.is_some());
            let finish_idx = finish_idx.unwrap_or(0);
            tx[10] = (finish_idx & 0xFF) as u8;
            tx[11] = (finish_idx >> 8) as u8;
        }

        let mut send = 0;
        match mode {
            GainSTMMode::PhaseDutyFull => {
                let d = &drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        tx[offset..].as_mut_ptr() as *mut LegacyDrive,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                send += 1;
            }
            GainSTMMode::PhaseFull => {
                let d = &drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        tx[offset..].as_mut_ptr() as *mut LegacyPhaseFull<0>,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                send += 1;
                if drives.len() > sent + 1 {
                    let d = &drives[sent + 1][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut LegacyPhaseFull<1>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
            }
            GainSTMMode::PhaseHalf => {
                let d = &drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        tx[offset..].as_mut_ptr() as *mut LegacyPhaseHalf<0>,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                send += 1;
                if drives.len() > sent + 1 {
                    let d = &drives[sent + 1][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut LegacyPhaseHalf<1>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
                if drives.len() > sent + 2 {
                    let d = &drives[sent + 2][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut LegacyPhaseHalf<2>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
                if drives.len() > sent + 3 {
                    let d = &drives[sent + 3][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut LegacyPhaseHalf<3>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
            }
        }
        f.set(GainSTMControlFlags::STM_END, sent + send == drives.len());

        sent_map.insert(device.idx(), sent + send);

        tx[1] = f.bits() | ((send as u8 - 1) & 0x03) << 6;

        if sent == 0 {
            Ok(std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<GainSTMControlFlags>()
            + std::mem::size_of::<GainSTMMode>()
            +  std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>() // finish idx
            +device.num_transducers() * std::mem::size_of::<LegacyDrive>())
        } else {
            Ok(std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<LegacyDrive>())
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pack_advanced(
        drives: &[HashMap<usize, Vec<Drive>>],
        remains: &HashMap<usize, usize>,
        sent_map: &mut HashMap<usize, usize>,
        mode: GainSTMMode,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        assert!(remains[&device.idx()] > 0);

        tx[0] = TypeTag::GainSTM as u8;

        let sent = sent_map[&device.idx()];
        let mut offset =
            std::mem::size_of::<TypeTag>() + std::mem::size_of::<GainSTMControlFlags>();
        if sent == 0 {
            offset += std::mem::size_of::<GainSTMMode>()
            + std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>(); // finish idx
        }
        assert!(tx.len() >= offset + device.num_transducers() * 2);

        let mut f = GainSTMControlFlags::NONE;
        f.set(GainSTMControlFlags::STM_BEGIN, sent == 0);
        f.set(GainSTMControlFlags::STM_END, remains[&device.idx()] == 1);

        if sent == 0 {
            let mode = mode as u16;
            tx[2] = (mode & 0xFF) as u8;
            tx[3] = (mode >> 8) as u8;

            let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;
            tx[4] = (freq_div & 0xFF) as u8;
            tx[5] = ((freq_div >> 8) & 0xFF) as u8;
            tx[6] = ((freq_div >> 16) & 0xFF) as u8;
            tx[7] = ((freq_div >> 24) & 0xFF) as u8;

            f.set(GainSTMControlFlags::USE_START_IDX, start_idx.is_some());
            let start_idx = start_idx.unwrap_or(0);
            tx[8] = (start_idx & 0xFF) as u8;
            tx[9] = (start_idx >> 8) as u8;

            f.set(GainSTMControlFlags::USE_FINISH_IDX, finish_idx.is_some());
            let finish_idx = finish_idx.unwrap_or(0);
            tx[10] = (finish_idx & 0xFF) as u8;
            tx[11] = (finish_idx >> 8) as u8;
        }

        match mode {
            GainSTMMode::PhaseDutyFull => {
                let d = &drives[sent / 2][&device.idx()];

                if sent % 2 == 0 {
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut AdvancedDrivePhase,
                            d.len(),
                        );
                        dst.iter_mut()
                            .zip(d.iter())
                            .zip(device.iter())
                            .for_each(|((d, s), tr)| d.set(s, tr.cycle()));
                    }
                } else {
                    f.set(GainSTMControlFlags::DUTY, true);

                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut AdvancedDriveDuty,
                            d.len(),
                        );
                        dst.iter_mut()
                            .zip(d.iter())
                            .zip(device.iter())
                            .for_each(|((d, s), tr)| d.set(s, tr.cycle()));
                    }
                }

                sent_map.insert(device.idx(), sent + 1);
            }
            GainSTMMode::PhaseFull => {
                let d = &drives[sent][&device.idx()];

                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        tx[offset..].as_mut_ptr() as *mut AdvancedDrivePhase,
                        d.len(),
                    );
                    dst.iter_mut()
                        .zip(d.iter())
                        .zip(device.iter())
                        .for_each(|((d, s), tr)| d.set(s, tr.cycle()));
                }

                sent_map.insert(device.idx(), sent + 1);
            }
            GainSTMMode::PhaseHalf => unreachable!(),
        }
        tx[1] = f.bits();

        if sent == 0 {
            Ok(std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<GainSTMControlFlags>()
            + std::mem::size_of::<GainSTMMode>()
            + std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>() // finish idx
            +device.num_transducers() * std::mem::size_of::<AdvancedDrivePhase>())
        } else {
            Ok(std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<u16>())
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pack_advanced_phase(
        drives: &[HashMap<usize, Vec<Drive>>],
        remains: &HashMap<usize, usize>,
        sent_map: &mut HashMap<usize, usize>,
        _mode: GainSTMMode,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        assert!(remains[&device.idx()] > 0);

        tx[0] = TypeTag::GainSTM as u8;

        let sent = sent_map[&device.idx()];
        let mut offset =
            std::mem::size_of::<TypeTag>() + std::mem::size_of::<GainSTMControlFlags>();
        if sent == 0 {
            offset += std::mem::size_of::<GainSTMMode>()
            +  std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>(); // finish idx
        }
        assert!(tx.len() >= offset + device.num_transducers() * 2);

        let mut f = GainSTMControlFlags::NONE;
        f.set(GainSTMControlFlags::STM_BEGIN, sent == 0);
        f.set(GainSTMControlFlags::STM_END, remains[&device.idx()] == 1);

        if sent == 0 {
            let mode = GainSTMMode::PhaseFull as u16;
            tx[2] = (mode & 0xFF) as u8;
            tx[3] = (mode >> 8) as u8;

            let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;
            tx[4] = (freq_div & 0xFF) as u8;
            tx[5] = ((freq_div >> 8) & 0xFF) as u8;
            tx[6] = ((freq_div >> 16) & 0xFF) as u8;
            tx[7] = ((freq_div >> 24) & 0xFF) as u8;

            f.set(GainSTMControlFlags::USE_START_IDX, start_idx.is_some());
            let start_idx = start_idx.unwrap_or(0);
            tx[8] = (start_idx & 0xFF) as u8;
            tx[9] = (start_idx >> 8) as u8;

            f.set(GainSTMControlFlags::USE_FINISH_IDX, finish_idx.is_some());
            let finish_idx = finish_idx.unwrap_or(0);
            tx[10] = (finish_idx & 0xFF) as u8;
            tx[11] = (finish_idx >> 8) as u8;
        }

        let d = &drives[sent][&device.idx()];

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                tx[offset..].as_mut_ptr() as *mut AdvancedDrivePhase,
                d.len(),
            );
            dst.iter_mut()
                .zip(d.iter())
                .zip(device.iter())
                .for_each(|((d, s), tr)| d.set(s, tr.cycle()));
        }

        sent_map.insert(device.idx(), sent + 1);

        tx[1] = f.bits();

        if sent == 0 {
            Ok(std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<GainSTMControlFlags>()
            + std::mem::size_of::<GainSTMMode>()
            + std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>() // finish idx
            +device.num_transducers() * std::mem::size_of::<AdvancedDrivePhase>())
        } else {
            Ok(std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<AdvancedDrivePhase>())
        }
    }

    pub fn init_legacy(
        gains: &Vec<G>,
        drives: &mut Vec<HashMap<usize, Vec<Drive>>>,
        remains: &mut HashMap<usize, usize>,
        sent: &mut HashMap<usize, usize>,
        _mode: GainSTMMode,
        freq_div: u32,
        geometry: &Geometry<T>,
    ) -> Result<(), AUTDInternalError> {
        if gains.len() < 2 || gains.len() > GAIN_STM_LEGACY_BUF_SIZE_MAX {
            return Err(AUTDInternalError::GainSTMLegacySizeOutOfRange(gains.len()));
        }
        if freq_div < SAMPLING_FREQ_DIV_MIN || freq_div > u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32 {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(freq_div));
        }

        *drives = gains
            .iter()
            .map(|g| g.calc(geometry, GainFilter::All))
            .collect::<Result<_, _>>()?;

        *remains = geometry
            .devices()
            .map(|device| (device.idx(), gains.len()))
            .collect();

        *sent = geometry.devices().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    pub fn init_advanced(
        gains: &Vec<G>,
        drives: &mut Vec<HashMap<usize, Vec<Drive>>>,
        remains: &mut HashMap<usize, usize>,
        sent: &mut HashMap<usize, usize>,
        mode: GainSTMMode,
        freq_div: u32,
        geometry: &Geometry<T>,
    ) -> Result<(), AUTDInternalError> {
        if gains.len() < 2 || gains.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(AUTDInternalError::GainSTMSizeOutOfRange(gains.len()));
        }
        if freq_div < SAMPLING_FREQ_DIV_MIN || freq_div > u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32 {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(freq_div));
        }

        match mode {
            GainSTMMode::PhaseDutyFull => {
                *remains = geometry
                    .devices()
                    .map(|device| (device.idx(), 2 * gains.len()))
                    .collect()
            }
            GainSTMMode::PhaseFull => {
                *remains = geometry
                    .devices()
                    .map(|device| (device.idx(), gains.len()))
                    .collect()
            }
            GainSTMMode::PhaseHalf => return Err(AUTDInternalError::GainSTMModeNotSupported(mode)),
        }

        *drives = gains
            .iter()
            .map(|g| g.calc(geometry, GainFilter::All))
            .collect::<Result<_, _>>()?;

        *sent = geometry.devices().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    pub fn init_advanced_phase(
        gains: &Vec<G>,
        drives: &mut Vec<HashMap<usize, Vec<Drive>>>,
        remains: &mut HashMap<usize, usize>,
        sent: &mut HashMap<usize, usize>,
        mode: GainSTMMode,
        freq_div: u32,
        geometry: &Geometry<T>,
    ) -> Result<(), AUTDInternalError> {
        if gains.len() < 2 || gains.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(AUTDInternalError::GainSTMSizeOutOfRange(gains.len()));
        }
        if freq_div < SAMPLING_FREQ_DIV_MIN || freq_div > u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32 {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(freq_div));
        }

        match mode {
            GainSTMMode::PhaseDutyFull | GainSTMMode::PhaseFull => {
                *remains = geometry
                    .devices()
                    .map(|device| (device.idx(), gains.len()))
                    .collect()
            }
            GainSTMMode::PhaseHalf => return Err(AUTDInternalError::GainSTMModeNotSupported(mode)),
        }

        *drives = gains
            .iter()
            .map(|g| g.calc(geometry, GainFilter::All))
            .collect::<Result<_, _>>()?;

        *sent = geometry.devices().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    pub fn commit_legacy(
        gains: &Vec<G>,
        remains: &mut HashMap<usize, usize>,
        sent: &HashMap<usize, usize>,
        _mode: GainSTMMode,
        device: &Device<T>,
    ) {
        remains.insert(device.idx(), gains.len() - sent[&device.idx()]);
    }

    pub fn commit_advanced(
        gains: &Vec<G>,
        remains: &mut HashMap<usize, usize>,
        sent: &HashMap<usize, usize>,
        mode: GainSTMMode,
        device: &Device<T>,
    ) {
        match mode {
            GainSTMMode::PhaseDutyFull => {
                remains.insert(device.idx(), 2 * gains.len() - sent[&device.idx()])
            }
            GainSTMMode::PhaseFull => {
                remains.insert(device.idx(), gains.len() - sent[&device.idx()])
            }
            GainSTMMode::PhaseHalf => unreachable!(),
        };
    }

    pub fn commit_advanced_phase(
        gains: &Vec<G>,
        remains: &mut HashMap<usize, usize>,
        sent: &HashMap<usize, usize>,
        _mode: GainSTMMode,
        device: &Device<T>,
    ) {
        remains.insert(device.idx(), gains.len() - sent[&device.idx()]);
    }

    pub fn required_size_impl(sent: &HashMap<usize, usize>, device: &Device<T>) -> usize {
        if sent[&device.idx()] == 0 {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + std::mem::size_of::<GainSTMMode>()
                + std::mem::size_of::<u32>() // freq_div
                + std::mem::size_of::<u16>() // start idx
                + std::mem::size_of::<u16>() // finish idx
                + device.num_transducers() * std::mem::size_of::<LegacyDrive>()
        } else {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<LegacyDrive>()
        }
    }
}

impl<G: Gain<LegacyTransducer>> Operation<LegacyTransducer> for GainSTMOp<LegacyTransducer, G> {
    fn pack(
        &mut self,
        device: &Device<LegacyTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        Self::pack_legacy(
            &self.drives,
            &self.remains,
            &mut self.sent,
            self.mode,
            self.freq_div,
            self.start_idx,
            self.finish_idx,
            device,
            tx,
        )
    }

    fn required_size(&self, device: &Device<LegacyTransducer>) -> usize {
        Self::required_size_impl(&self.sent, device)
    }

    fn init(&mut self, geometry: &Geometry<LegacyTransducer>) -> Result<(), AUTDInternalError> {
        Self::init_legacy(
            &self.gains,
            &mut self.drives,
            &mut self.remains,
            &mut self.sent,
            self.mode,
            self.freq_div,
            geometry,
        )
    }

    fn remains(&self, device: &Device<LegacyTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<LegacyTransducer>) {
        Self::commit_legacy(
            &self.gains,
            &mut self.remains,
            &self.sent,
            self.mode,
            device,
        );
    }
}

impl<G: Gain<AdvancedTransducer>> Operation<AdvancedTransducer>
    for GainSTMOp<AdvancedTransducer, G>
{
    fn pack(
        &mut self,
        device: &Device<AdvancedTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        Self::pack_advanced(
            &self.drives,
            &self.remains,
            &mut self.sent,
            self.mode,
            self.freq_div,
            self.start_idx,
            self.finish_idx,
            device,
            tx,
        )
    }

    fn required_size(&self, device: &Device<AdvancedTransducer>) -> usize {
        Self::required_size_impl(&self.sent, device)
    }

    fn init(&mut self, geometry: &Geometry<AdvancedTransducer>) -> Result<(), AUTDInternalError> {
        Self::init_advanced(
            &self.gains,
            &mut self.drives,
            &mut self.remains,
            &mut self.sent,
            self.mode,
            self.freq_div,
            geometry,
        )
    }

    fn remains(&self, device: &Device<AdvancedTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<AdvancedTransducer>) {
        Self::commit_advanced(
            &self.gains,
            &mut self.remains,
            &self.sent,
            self.mode,
            device,
        );
    }
}

impl<G: Gain<AdvancedPhaseTransducer>> Operation<AdvancedPhaseTransducer>
    for GainSTMOp<AdvancedPhaseTransducer, G>
{
    fn pack(
        &mut self,
        device: &Device<AdvancedPhaseTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        Self::pack_advanced_phase(
            &self.drives,
            &self.remains,
            &mut self.sent,
            self.mode,
            self.freq_div,
            self.start_idx,
            self.finish_idx,
            device,
            tx,
        )
    }

    fn required_size(&self, device: &Device<AdvancedPhaseTransducer>) -> usize {
        Self::required_size_impl(&self.sent, device)
    }

    fn init(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<(), AUTDInternalError> {
        Self::init_advanced_phase(
            &self.gains,
            &mut self.drives,
            &mut self.remains,
            &mut self.sent,
            self.mode,
            self.freq_div,
            geometry,
        )
    }

    fn remains(&self, device: &Device<AdvancedPhaseTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<AdvancedPhaseTransducer>) {
        Self::commit_advanced_phase(
            &self.gains,
            &mut self.remains,
            &self.sent,
            self.mode,
            device,
        );
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::{
        defined::PI,
        fpga::{GAIN_STM_BUF_SIZE_MAX, SAMPLING_FREQ_DIV_MIN},
        geometry::{tests::create_geometry, LegacyTransducer},
        operation::tests::{NullGain, TestGain},
    };

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn gain_stm_legacy_phase_duty_full_op() {
        const GAIN_STM_SIZE: usize = 3;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);
        let mut op =
            GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseDutyFull, freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 0);

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseDutyFull as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseDutyFull as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[0][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g));
                    assert_eq!(d[1], LegacyDrive::to_duty(g));
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 0);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[1][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g));
                    assert_eq!(d[1], LegacyDrive::to_duty(g));
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 0);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[2][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g));
                    assert_eq!(d[1], LegacyDrive::to_duty(g));
                })
        });
    }

    #[test]
    fn gain_stm_legacy_phase_full_op() {
        const GAIN_STM_SIZE: usize = 5;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);
        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseFull, freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 1);

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseFull as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseFull as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(gain_data[1][&dev.idx()].iter())
                .for_each(|((d, g0), g1)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g0));
                    assert_eq!(d[1], LegacyDrive::to_phase(g1));
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 4));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 1);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[2][&dev.idx()].iter())
                .zip(gain_data[3][&dev.idx()].iter())
                .for_each(|((d, g0), g1)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g0));
                    assert_eq!(d[1], LegacyDrive::to_phase(g1));
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 0);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[4][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g));
                })
        });
    }

    #[test]
    fn gain_stm_legacy_phase_half_op() {
        const GAIN_STM_SIZE: usize = 11;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);
        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseHalf, freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 4));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 3);

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseHalf as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseHalf as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(gain_data[1][&dev.idx()].iter())
                .zip(gain_data[2][&dev.idx()].iter())
                .zip(gain_data[3][&dev.idx()].iter())
                .for_each(|((((d, g0), g1), g2), g3)| {
                    assert_eq!(d[0] & 0x0F, LegacyDrive::to_phase(g0) >> 4);
                    assert_eq!(d[0] >> 4, LegacyDrive::to_phase(g1) >> 4);
                    assert_eq!(d[1] & 0x0F, LegacyDrive::to_phase(g2) >> 4);
                    assert_eq!(d[1] >> 4, LegacyDrive::to_phase(g3) >> 4);
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 8));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 3);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[4][&dev.idx()].iter())
                .zip(gain_data[5][&dev.idx()].iter())
                .zip(gain_data[6][&dev.idx()].iter())
                .zip(gain_data[7][&dev.idx()].iter())
                .for_each(|((((d, g0), g1), g2), g3)| {
                    assert_eq!(d[0] & 0x0F, LegacyDrive::to_phase(g0) >> 4);
                    assert_eq!(d[0] >> 4, LegacyDrive::to_phase(g1) >> 4);
                    assert_eq!(d[1] & 0x0F, LegacyDrive::to_phase(g2) >> 4);
                    assert_eq!(d[1] >> 4, LegacyDrive::to_phase(g3) >> 4);
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 2);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[8][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0] & 0x0F, LegacyDrive::to_phase(g) >> 4);
                })
        });
    }

    #[test]
    fn gain_stm_legacy_op_idx() {
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let start_idx = rng.gen_range(0..2_u16);
        let finish_idx = rng.gen_range(0..2_u16);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..2)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            Some(finish_idx),
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], (finish_idx >> 8) as u8);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            None,
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            Some(finish_idx),
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], (finish_idx >> 8) as u8);
        });
    }

    #[test]
    fn gain_stm_legacy_op_buffer_out_of_range() {
        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let gains: Vec<NullGain> = (0..GAIN_STM_LEGACY_BUF_SIZE_MAX)
            .map(|_| NullGain {})
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains,
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let gains: Vec<NullGain> = (0..GAIN_STM_LEGACY_BUF_SIZE_MAX + 1)
            .map(|_| NullGain {})
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains,
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());
    }

    #[test]
    fn gain_stm_legacy_op_freq_div_out_of_range() {
        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let gains: Vec<NullGain> = (0..2).map(|_| NullGain {}).collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN - 1,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32 + 1,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase_duty_full_op() {
        const GAIN_STM_SIZE: usize = 2;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<AdvancedTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);
        let mut op =
            GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseDutyFull, freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 2 * GAIN_STM_SIZE));

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 2 * GAIN_STM_SIZE - 1));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseDutyFull as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseDutyFull as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 2 * GAIN_STM_SIZE - 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDriveDuty>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let duty = AdvancedDriveDuty::to_duty(g, tr.cycle());
                    assert_eq!(d[0], (duty & 0xFF) as u8);
                    assert_eq!(d[1], (duty >> 8) as u8);
                })
        });

        // Third frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 2 * GAIN_STM_SIZE - 3));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[1][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDriveDuty>())
                .zip(gain_data[1][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let duty = AdvancedDriveDuty::to_duty(g, tr.cycle());
                    assert_eq!(d[0], (duty & 0xFF) as u8);
                    assert_eq!(d[1], (duty >> 8) as u8);
                })
        });
    }

    #[test]
    fn gain_stm_advanced_phase_full_op() {
        const GAIN_STM_SIZE: usize = 3;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<AdvancedTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);
        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseFull, freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseFull as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseFull as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[1][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[2][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });
    }

    #[test]
    fn gain_stm_advanced_phase_half_op() {
        const GAIN_STM_SIZE: usize = 2;

        let geometry = create_geometry::<AdvancedTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);

        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseHalf, freq_div, None, None);

        assert!(op.init(&geometry).is_err());
    }

    #[test]
    fn gain_stm_advanced_op_idx() {
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<AdvancedTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let start_idx = rng.gen_range(0..2_u16);
        let finish_idx = rng.gen_range(0..2_u16);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..2)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            Some(finish_idx),
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], (finish_idx >> 8) as u8);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            None,
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            Some(finish_idx),
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], (finish_idx >> 8) as u8);
        });
    }

    #[test]
    fn gain_stm_advanced_op_buffer_out_of_range() {
        let geometry = create_geometry::<AdvancedTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let gains: Vec<NullGain> = (0..GAIN_STM_BUF_SIZE_MAX).map(|_| NullGain {}).collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains,
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let gains: Vec<NullGain> = (0..GAIN_STM_BUF_SIZE_MAX + 1)
            .map(|_| NullGain {})
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains,
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());
    }

    #[test]
    fn gain_stm_advanced_op_freq_div_out_of_range() {
        let geometry = create_geometry::<AdvancedTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let gains: Vec<NullGain> = (0..2).map(|_| NullGain {}).collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN - 1,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32 + 1,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase_phase_duty_full_op() {
        const GAIN_STM_SIZE: usize = 3;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);
        let mut op =
            GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseDutyFull, freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseFull as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseFull as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[1][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[2][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });
    }

    #[test]
    fn gain_stm_advanced_phase_phase_full_op() {
        const GAIN_STM_SIZE: usize = 3;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);
        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseFull, freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseFull as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseFull as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[1][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<AdvancedDrivePhase>())
                .zip(gain_data[2][&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });
    }

    #[test]
    fn gain_stm_advanced_phase_phase_half_op() {
        const GAIN_STM_SIZE: usize = 2;

        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);

        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseHalf, freq_div, None, None);

        assert!(op.init(&geometry).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase_op_idx() {
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let start_idx = rng.gen_range(0..2_u16);
        let finish_idx = rng.gen_range(0..2_u16);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: rng.gen_range(0.0..1.0),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..2)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            Some(finish_idx),
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], (finish_idx >> 8) as u8);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            None,
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            Some(finish_idx),
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], (finish_idx >> 8) as u8);
        });
    }

    #[test]
    fn gain_stm_advanced_phase_op_buffer_out_of_range() {
        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let gains: Vec<NullGain> = (0..GAIN_STM_BUF_SIZE_MAX).map(|_| NullGain {}).collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains,
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let gains: Vec<NullGain> = (0..GAIN_STM_BUF_SIZE_MAX + 1)
            .map(|_| NullGain {})
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains,
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase_op_freq_div_out_of_range() {
        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let gains: Vec<NullGain> = (0..2).map(|_| NullGain {}).collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN - 1,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32 + 1,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());
    }
}
