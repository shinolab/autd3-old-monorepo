/*
 * File: gain.rs
 * Project: stm
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
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
        AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive, GAIN_STM_BUF_SIZE_MAX,
        GAIN_STM_LEGACY_BUF_SIZE_MAX, SAMPLING_FREQ_DIV_MIN,
    },
    geometry::{AdvancedPhaseTransducer, AdvancedTransducer, Device, LegacyTransducer, Transducer},
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
        const IGNORE_DUTY     = 1 << 6;
        const PHASE_COMPRESS  = 1 << 7;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GainSTMMode {
    #[default]
    PhaseDutyFull,
    PhaseFull,
    PhaseHalf,
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
}

impl<G: Gain<LegacyTransducer>> Operation<LegacyTransducer> for GainSTMOp<LegacyTransducer, G> {
    fn pack(
        &mut self,
        device: &Device<LegacyTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        assert!(self.remains[&device.idx()] > 0);

        tx[0] = TypeTag::GainSTM as u8;

        let sent = self.sent[&device.idx()];
        let mut offset =
            std::mem::size_of::<TypeTag>() + std::mem::size_of::<GainSTMControlFlags>();
        if sent == 0 {
            offset += std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>(); // finish idx
        }
        assert!(tx.len() >= offset + device.num_transducers() * std::mem::size_of::<LegacyDrive>());

        let mut f = GainSTMControlFlags::LEGACY;
        f.set(GainSTMControlFlags::STM_BEGIN, sent == 0);
        f.set(GainSTMControlFlags::STM_END, sent + 1 == self.gains.len());

        if sent == 0 {
            tx[2] = (self.freq_div & 0xFF) as u8;
            tx[3] = ((self.freq_div >> 8) & 0xFF) as u8;
            tx[4] = ((self.freq_div >> 16) & 0xFF) as u8;
            tx[5] = ((self.freq_div >> 24) & 0xFF) as u8;

            let start_idx = self.start_idx.unwrap_or(0);
            tx[6] = (start_idx & 0xFF) as u8;
            tx[7] = (start_idx >> 8) as u8;
            f.set(GainSTMControlFlags::USE_START_IDX, self.start_idx.is_some());

            let finish_idx = self.finish_idx.unwrap_or(0);
            tx[8] = (finish_idx & 0xFF) as u8;
            tx[9] = (finish_idx >> 8) as u8;
            f.set(
                GainSTMControlFlags::USE_FINISH_IDX,
                self.finish_idx.is_some(),
            );
        }

        match self.mode {
            GainSTMMode::PhaseDutyFull => {
                let d = &self.drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        (&mut tx[offset..]).as_mut_ptr() as *mut LegacyDrive,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                self.sent.insert(device.idx(), sent + 1);
            }
            GainSTMMode::PhaseFull => {
                f.set(GainSTMControlFlags::IGNORE_DUTY, true);

                let d = &self.drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        (&mut tx[offset..]).as_mut_ptr() as *mut LegacyPhaseFull<0>,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                let mut send = 1;
                if self.drives.len() > sent + 1 {
                    let d = &self.drives[sent + 1][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            (&mut tx[offset..]).as_mut_ptr() as *mut LegacyPhaseFull<1>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
                self.sent.insert(device.idx(), sent + send);
            }
            GainSTMMode::PhaseHalf => {
                f.set(GainSTMControlFlags::IGNORE_DUTY, true);
                f.set(GainSTMControlFlags::PHASE_COMPRESS, true);

                let d = &self.drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        (&mut tx[offset..]).as_mut_ptr() as *mut LegacyPhaseHalf<0>,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                let mut send = 1;
                if self.drives.len() > sent + 1 {
                    let d = &self.drives[sent + 1][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            (&mut tx[offset..]).as_mut_ptr() as *mut LegacyPhaseHalf<1>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
                if self.drives.len() > sent + 2 {
                    let d = &self.drives[sent + 2][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            (&mut tx[offset..]).as_mut_ptr() as *mut LegacyPhaseHalf<2>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
                if self.drives.len() > sent + 3 {
                    let d = &self.drives[sent + 3][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            (&mut tx[offset..]).as_mut_ptr() as *mut LegacyPhaseHalf<3>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
                self.sent.insert(device.idx(), sent + send);
            }
        }
        tx[1] = f.bits();

        if sent == 0 {
            Ok(std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<GainSTMControlFlags>()
            + std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>() // finish idx
            +device.num_transducers() * std::mem::size_of::<LegacyDrive>())
        } else {
            Ok(std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<LegacyDrive>())
        }
    }

    fn required_size(&self, device: &Device<LegacyTransducer>) -> usize {
        if self.sent[&device.idx()] == 0 {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
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

    fn init(&mut self, devices: &[&Device<LegacyTransducer>]) -> Result<(), AUTDInternalError> {
        if self.gains.len() < 2 || self.gains.len() > GAIN_STM_LEGACY_BUF_SIZE_MAX {
            return Err(AUTDInternalError::GainSTMLegacySizeOutOfRange(
                self.gains.len(),
            ));
        }
        if self.freq_div < SAMPLING_FREQ_DIV_MIN {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(self.freq_div));
        }

        self.drives = self
            .gains
            .iter()
            .map(|g| g.calc(devices, GainFilter::All))
            .collect::<Result<_, _>>()?;

        self.remains = devices
            .iter()
            .map(|device| (device.idx(), self.gains.len()))
            .collect();

        self.sent = devices.iter().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    fn remains(&self, device: &Device<LegacyTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<LegacyTransducer>) {
        self.remains
            .insert(device.idx(), self.gains.len() - self.sent[&device.idx()]);
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
        assert!(self.remains[&device.idx()] > 0);

        tx[0] = TypeTag::GainSTM as u8;

        let sent = self.sent[&device.idx()];
        let mut offset =
            std::mem::size_of::<TypeTag>() + std::mem::size_of::<GainSTMControlFlags>();
        if sent == 0 {
            offset += std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>(); // finish idx
        }
        assert!(tx.len() >= offset + device.num_transducers() * 2);

        let mut f = GainSTMControlFlags::NONE;
        f.set(GainSTMControlFlags::STM_BEGIN, sent == 0);
        f.set(
            GainSTMControlFlags::STM_END,
            self.remains[&device.idx()] == 1,
        );

        if sent == 0 {
            tx[2] = (self.freq_div & 0xFF) as u8;
            tx[3] = ((self.freq_div >> 8) & 0xFF) as u8;
            tx[4] = ((self.freq_div >> 16) & 0xFF) as u8;
            tx[5] = ((self.freq_div >> 24) & 0xFF) as u8;

            let start_idx = self.start_idx.unwrap_or(0);
            tx[6] = (start_idx & 0xFF) as u8;
            tx[7] = (start_idx >> 8) as u8;
            f.set(GainSTMControlFlags::USE_START_IDX, self.start_idx.is_some());

            let finish_idx = self.finish_idx.unwrap_or(0);
            tx[8] = (finish_idx & 0xFF) as u8;
            tx[9] = (finish_idx >> 8) as u8;
            f.set(
                GainSTMControlFlags::USE_FINISH_IDX,
                self.finish_idx.is_some(),
            );
        }

        match self.mode {
            GainSTMMode::PhaseDutyFull => {
                let d = &self.drives[sent / 2][&device.idx()];

                if sent % 2 == 0 {
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            (&mut tx[offset..]).as_mut_ptr() as *mut AdvancedDrivePhase,
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
                            (&mut tx[offset..]).as_mut_ptr() as *mut AdvancedDriveDuty,
                            d.len(),
                        );
                        dst.iter_mut()
                            .zip(d.iter())
                            .zip(device.iter())
                            .for_each(|((d, s), tr)| d.set(s, tr.cycle()));
                    }
                }

                self.sent.insert(device.idx(), sent + 1);
            }
            GainSTMMode::PhaseFull => {
                f.set(GainSTMControlFlags::IGNORE_DUTY, true);

                let d = &self.drives[sent][&device.idx()];

                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        (&mut tx[offset..]).as_mut_ptr() as *mut AdvancedDrivePhase,
                        d.len(),
                    );
                    dst.iter_mut()
                        .zip(d.iter())
                        .zip(device.iter())
                        .for_each(|((d, s), tr)| d.set(s, tr.cycle()));
                }

                self.sent.insert(device.idx(), sent + 1);
            }
            GainSTMMode::PhaseHalf => unreachable!(),
        }
        tx[1] = f.bits();

        if sent == 0 {
            Ok(std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<GainSTMControlFlags>()
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

    fn required_size(&self, device: &Device<AdvancedTransducer>) -> usize {
        if self.sent[&device.idx()] == 0 {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + std::mem::size_of::<u32>() // freq_div
                + std::mem::size_of::<u16>() // start idx
                + std::mem::size_of::<u16>() // finish idx
                + device.num_transducers() * std::mem::size_of::<AdvancedDrivePhase>()
        } else {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<u16>()
        }
    }

    fn init(&mut self, devices: &[&Device<AdvancedTransducer>]) -> Result<(), AUTDInternalError> {
        if self.gains.len() < 2 || self.gains.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(AUTDInternalError::GainSTMSizeOutOfRange(self.gains.len()));
        }
        if self.freq_div < SAMPLING_FREQ_DIV_MIN {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(self.freq_div));
        }

        match self.mode {
            GainSTMMode::PhaseDutyFull => {
                self.remains = devices
                    .iter()
                    .map(|device| (device.idx(), 2 * self.gains.len()))
                    .collect()
            }
            GainSTMMode::PhaseFull => {
                self.remains = devices
                    .iter()
                    .map(|device| (device.idx(), self.gains.len()))
                    .collect()
            }
            GainSTMMode::PhaseHalf => {
                return Err(AUTDInternalError::GainSTMModeNotSupported(self.mode))
            }
        }

        self.drives = self
            .gains
            .iter()
            .map(|g| g.calc(devices, GainFilter::All))
            .collect::<Result<_, _>>()?;

        self.sent = devices.iter().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    fn remains(&self, device: &Device<AdvancedTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<AdvancedTransducer>) {
        match self.mode {
            GainSTMMode::PhaseDutyFull => self.remains.insert(
                device.idx(),
                2 * self.gains.len() - self.sent[&device.idx()],
            ),
            GainSTMMode::PhaseFull => self
                .remains
                .insert(device.idx(), self.gains.len() - self.sent[&device.idx()]),
            GainSTMMode::PhaseHalf => unreachable!(),
        };
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
        assert!(self.remains[&device.idx()] > 0);

        tx[0] = TypeTag::GainSTM as u8;

        let sent = self.sent[&device.idx()];
        let mut offset =
            std::mem::size_of::<TypeTag>() + std::mem::size_of::<GainSTMControlFlags>();
        if sent == 0 {
            offset += std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>(); // finish idx
        }
        assert!(tx.len() >= offset + device.num_transducers() * 2);

        let mut f = GainSTMControlFlags::NONE;
        f.set(GainSTMControlFlags::STM_BEGIN, sent == 0);
        f.set(
            GainSTMControlFlags::STM_END,
            self.remains[&device.idx()] == 1,
        );

        if sent == 0 {
            tx[2] = (self.freq_div & 0xFF) as u8;
            tx[3] = ((self.freq_div >> 8) & 0xFF) as u8;
            tx[4] = ((self.freq_div >> 16) & 0xFF) as u8;
            tx[5] = ((self.freq_div >> 24) & 0xFF) as u8;

            let start_idx = self.start_idx.unwrap_or(0);
            tx[6] = (start_idx & 0xFF) as u8;
            tx[7] = (start_idx >> 8) as u8;
            f.set(GainSTMControlFlags::USE_START_IDX, self.start_idx.is_some());

            let finish_idx = self.finish_idx.unwrap_or(0);
            tx[8] = (finish_idx & 0xFF) as u8;
            tx[9] = (finish_idx >> 8) as u8;
            f.set(
                GainSTMControlFlags::USE_FINISH_IDX,
                self.finish_idx.is_some(),
            );
        }

        f.set(GainSTMControlFlags::IGNORE_DUTY, true);

        let d = &self.drives[sent][&device.idx()];

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                (&mut tx[offset..]).as_mut_ptr() as *mut AdvancedDrivePhase,
                d.len(),
            );
            dst.iter_mut()
                .zip(d.iter())
                .zip(device.iter())
                .for_each(|((d, s), tr)| d.set(s, tr.cycle()));
        }

        self.sent.insert(device.idx(), sent + 1);

        tx[1] = f.bits();

        if sent == 0 {
            Ok(std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<GainSTMControlFlags>()
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

    fn required_size(&self, device: &Device<AdvancedPhaseTransducer>) -> usize {
        if self.sent[&device.idx()] == 0 {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + std::mem::size_of::<u32>() // freq_div
                + std::mem::size_of::<u16>() // start idx
                + std::mem::size_of::<u16>() // finish idx
                + device.num_transducers() * std::mem::size_of::<AdvancedDrivePhase>()
        } else {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<AdvancedDrivePhase>()
        }
    }

    fn init(
        &mut self,
        devices: &[&Device<AdvancedPhaseTransducer>],
    ) -> Result<(), AUTDInternalError> {
        if self.gains.len() < 2 || self.gains.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(AUTDInternalError::GainSTMSizeOutOfRange(self.gains.len()));
        }
        if self.freq_div < SAMPLING_FREQ_DIV_MIN {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(self.freq_div));
        }

        match self.mode {
            GainSTMMode::PhaseDutyFull | GainSTMMode::PhaseFull => {
                self.remains = devices
                    .iter()
                    .map(|device| (device.idx(), self.gains.len()))
                    .collect()
            }
            GainSTMMode::PhaseHalf => {
                return Err(AUTDInternalError::GainSTMModeNotSupported(self.mode))
            }
        }

        self.drives = self
            .gains
            .iter()
            .map(|g| g.calc(devices, GainFilter::All))
            .collect::<Result<_, _>>()?;

        self.sent = devices.iter().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    fn remains(&self, device: &Device<AdvancedPhaseTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<AdvancedPhaseTransducer>) {
        self.remains
            .insert(device.idx(), self.gains.len() - self.sent[&device.idx()]);
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::{
        datagram::GainAsAny,
        defined::PI,
        fpga::{GAIN_STM_BUF_SIZE_MAX, SAMPLING_FREQ_DIV_MIN},
        geometry::{tests::create_device, LegacyTransducer},
    };

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[derive(Clone)]
    pub struct TestGain {
        pub data: HashMap<usize, Vec<Drive>>,
    }

    impl GainAsAny for TestGain {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl<T: Transducer> Gain<T> for TestGain {
        fn calc(
            &self,
            _devices: &[&Device<T>],
            _filter: GainFilter,
        ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
            Ok(self.data.clone())
        }
    }

    #[test]
    fn gain_stm_legacy_phase_duty_full_op() {
        const GAIN_STM_SIZE: usize = 3;
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op =
            GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseDutyFull, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        // First frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 10 + NUM_TRANS_IN_UNIT * 2));

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(10 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(!flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 2], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 4],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);

            tx[FRAME_SIZE * dev.idx() + 10..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[0][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g));
                    assert_eq!(d[1], LegacyDrive::to_duty(g));
                })
        });

        // Second frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(!flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[1][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g));
                    assert_eq!(d[1], LegacyDrive::to_duty(g));
                })
        });

        // Final frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(!flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseFull, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 10 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(10 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 2], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 4],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);

            tx[FRAME_SIZE * dev.idx() + 10..]
                .chunks(std::mem::size_of::<LegacyDrive>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(gain_data[1][&dev.idx()].iter())
                .for_each(|((d, g0), g1)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g0));
                    assert_eq!(d[1], LegacyDrive::to_phase(g1));
                })
        });

        // Second frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 4));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        const GAIN_STM_SIZE: usize = 9;
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseHalf, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 10 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(10 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 4));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 2], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 4],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);

            tx[FRAME_SIZE * dev.idx() + 10..]
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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 8));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let start_idx = rng.gen_range(0..2 as u16);
        let finish_idx = rng.gen_range(0..2 as u16);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                devices
                    .iter()
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
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (finish_idx >> 8) as u8);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            Some(finish_idx),
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (finish_idx >> 8) as u8);
        });
    }

    #[test]
    fn gain_stm_legacy_op_buffer_out_of_range() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_LEGACY_BUF_SIZE_MAX)
            .map(|_| {
                devices
                    .iter()
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
        let gains: Vec<TestGain> = (0..GAIN_STM_LEGACY_BUF_SIZE_MAX)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_LEGACY_BUF_SIZE_MAX + 1)
            .map(|_| {
                devices
                    .iter()
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
        let gains: Vec<TestGain> = (0..GAIN_STM_LEGACY_BUF_SIZE_MAX + 1)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn gain_stm_legacy_op_freq_div_out_of_range() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                devices
                    .iter()
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
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN - 1,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase_duty_full_op() {
        const GAIN_STM_SIZE: usize = 2;
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op =
            GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseDutyFull, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 2 * GAIN_STM_SIZE));

        // First frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 10 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(10 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 2 * GAIN_STM_SIZE - 1));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(!flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 2], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 4],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);

            tx[FRAME_SIZE * dev.idx() + 10..]
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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 2 * GAIN_STM_SIZE - 2));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(!flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 2 * GAIN_STM_SIZE - 3));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(!flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(!flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseFull, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 10 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(10 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 2], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 4],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);

            tx[FRAME_SIZE * dev.idx() + 10..]
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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseHalf, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn gain_stm_advanced_op_idx() {
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let start_idx = rng.gen_range(0..2 as u16);
        let finish_idx = rng.gen_range(0..2 as u16);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                devices
                    .iter()
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
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (finish_idx >> 8) as u8);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            Some(finish_idx),
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (finish_idx >> 8) as u8);
        });
    }

    #[test]
    fn gain_stm_advanced_op_buffer_out_of_range() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_BUF_SIZE_MAX)
            .map(|_| {
                devices
                    .iter()
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
        let gains: Vec<TestGain> = (0..GAIN_STM_BUF_SIZE_MAX)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_BUF_SIZE_MAX + 1)
            .map(|_| {
                devices
                    .iter()
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
        let gains: Vec<TestGain> = (0..GAIN_STM_BUF_SIZE_MAX + 1)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn gain_stm_advanced_op_freq_div_out_of_range() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                devices
                    .iter()
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
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN - 1,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase_phase_duty_full_op() {
        const GAIN_STM_SIZE: usize = 3;
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedPhaseTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op =
            GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseDutyFull, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 10 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(10 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 2], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 4],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);

            tx[FRAME_SIZE * dev.idx() + 10..]
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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedPhaseTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseFull, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 10 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(10 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 2], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 4],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);

            tx[FRAME_SIZE * dev.idx() + 10..]
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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::LEGACY));
            assert!(!flag.contains(GainSTMControlFlags::DUTY));
            assert!(!flag.contains(GainSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(GainSTMControlFlags::STM_END));
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));
            assert!(flag.contains(GainSTMControlFlags::IGNORE_DUTY));
            assert!(!flag.contains(GainSTMControlFlags::PHASE_COMPRESS));

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

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedPhaseTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                devices
                    .iter()
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

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = GainSTMOp::<_, _>::new(gains, GainSTMMode::PhaseHalf, freq_div, None, None);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase_op_idx() {
        const FRAME_SIZE: usize = 10 + NUM_TRANS_IN_UNIT * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedPhaseTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let start_idx = rng.gen_range(0..2 as u16);
        let finish_idx = rng.gen_range(0..2 as u16);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                devices
                    .iter()
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
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (finish_idx >> 8) as u8);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
        });

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            Some(finish_idx),
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices.iter().for_each(|dev| {
            let flag = GainSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(GainSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(GainSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 6], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 7], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (finish_idx >> 8) as u8);
        });
    }

    #[test]
    fn gain_stm_advanced_phase_op_buffer_out_of_range() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedPhaseTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_BUF_SIZE_MAX)
            .map(|_| {
                devices
                    .iter()
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
        let gains: Vec<TestGain> = (0..GAIN_STM_BUF_SIZE_MAX)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_BUF_SIZE_MAX + 1)
            .map(|_| {
                devices
                    .iter()
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
        let gains: Vec<TestGain> = (0..GAIN_STM_BUF_SIZE_MAX + 1)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase_op_freq_div_out_of_range() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<AdvancedPhaseTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                devices
                    .iter()
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
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            u32::MAX,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut op = GainSTMOp::<_, _>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN - 1,
            None,
            None,
        );
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }
}
