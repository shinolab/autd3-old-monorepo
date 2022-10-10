/*
 * File: gain.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::marker::PhantomData;

use crate::{
    gain::Gain,
    geometry::{Geometry, LegacyTransducer, NormalPhaseTransducer, NormalTransducer, Transducer},
    interface::{DatagramBody, Empty, Filled, Sendable},
};

use anyhow::{Ok, Result};
use autd3_driver::{Drive, Mode, TxDatagram, FPGA_CLK_FREQ, STM_SAMPLING_FREQ_DIV_MIN};

use super::STM;

pub struct GainSTM<T: Transducer> {
    gains: Vec<Vec<Drive>>,
    sample_freq_div: u32,
    next_duty: bool,
    sent: usize,
    mode: Mode,
    _t: PhantomData<T>,
}

impl<T: Transducer> GainSTM<T> {
    pub fn new() -> Self {
        Self {
            gains: vec![],
            sample_freq_div: 4096,
            next_duty: false,
            sent: 0,
            mode: Mode::PhaseDutyFull,
            _t: PhantomData,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn add<G: Gain<T>>(&mut self, gain: G, geometry: &Geometry<T>) -> Result<()> {
        if self.gains.len() + 1 > T::gain_stm_max() {
            return Err(autd3_driver::FPGAError::GainSTMOutOfBuffer(
                self.gains.len() + 1,
                T::gain_stm_max(),
            )
            .into());
        }

        let mut gain = gain;

        gain.build(geometry)?;

        let drives = gain.take_drives();

        self.gains.push(drives);
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.gains.len()
    }
}

impl<T: Transducer> Default for GainSTM<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl DatagramBody<LegacyTransducer> for GainSTM<LegacyTransducer> {
    fn init(&mut self) -> Result<()> {
        self.sent = 0;
        Ok(())
    }

    fn pack(&mut self, _geometry: &Geometry<LegacyTransducer>, tx: &mut TxDatagram) -> Result<()> {
        autd3_driver::gain_stm_legacy_head(tx);

        if DatagramBody::<LegacyTransducer>::is_finished(self) {
            return Ok(());
        }

        let is_first_frame = self.sent == 0;

        if is_first_frame {
            autd3_driver::gain_stm_legacy_body(
                &[],
                is_first_frame,
                self.sample_freq_div,
                false,
                self.mode,
                tx,
            )?;
            self.sent += 1;
            return Ok(());
        }

        match self.mode {
            Mode::PhaseDutyFull => {
                let is_last_frame = self.sent + 1 == self.gains.len() + 1;
                autd3_driver::gain_stm_legacy_body(
                    &[&self.gains[self.sent - 1]],
                    is_first_frame,
                    self.sample_freq_div,
                    is_last_frame,
                    self.mode,
                    tx,
                )?;
                self.sent += 1;
            }
            Mode::PhaseFull => {
                let is_last_frame = self.sent + 2 >= self.gains.len() + 1;
                autd3_driver::gain_stm_legacy_body(
                    &[
                        &self.gains[self.sent - 1],
                        self.gains.get(self.sent + 1 - 1).unwrap_or(&Vec::new()),
                    ],
                    is_first_frame,
                    self.sample_freq_div,
                    is_last_frame,
                    self.mode,
                    tx,
                )?;
                self.sent += 2;
            }
            Mode::PhaseHalf => {
                let is_last_frame = self.sent + 4 >= self.gains.len() + 1;
                autd3_driver::gain_stm_legacy_body(
                    &[
                        &self.gains[self.sent - 1],
                        self.gains.get(self.sent + 1 - 1).unwrap_or(&Vec::new()),
                        self.gains.get(self.sent + 2 - 1).unwrap_or(&Vec::new()),
                        self.gains.get(self.sent + 3 - 1).unwrap_or(&Vec::new()),
                    ],
                    is_first_frame,
                    self.sample_freq_div,
                    is_last_frame,
                    self.mode,
                    tx,
                )?;
                self.sent += 4;
            }
        }

        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.sent > self.gains.len()
    }
}

impl DatagramBody<NormalTransducer> for GainSTM<NormalTransducer> {
    fn init(&mut self) -> Result<()> {
        self.sent = 0;
        self.next_duty = false;
        Ok(())
    }

    fn pack(&mut self, _geometry: &Geometry<NormalTransducer>, tx: &mut TxDatagram) -> Result<()> {
        autd3_driver::gain_stm_normal_head(tx);

        if DatagramBody::<NormalTransducer>::is_finished(self) {
            return Ok(());
        }

        let is_first_frame = self.sent == 0;
        let is_last_frame = if self.mode == Mode::PhaseDutyFull {
            self.sent + 1 == self.gains.len() * 2 + 1
        } else {
            self.sent + 1 == self.gains.len() + 1
        };

        if is_first_frame {
            autd3_driver::gain_stm_normal_phase_body(
                &[],
                is_first_frame,
                self.sample_freq_div,
                self.mode,
                is_last_frame,
                tx,
            )?;
            self.sent += 1;
            return Ok(());
        }

        if !self.next_duty {
            let idx = if self.mode == Mode::PhaseDutyFull {
                (self.sent - 1) / 2
            } else {
                self.sent - 1
            };
            autd3_driver::gain_stm_normal_phase_body(
                &self.gains[idx],
                is_first_frame,
                self.sample_freq_div,
                self.mode,
                is_last_frame,
                tx,
            )?;
        } else {
            autd3_driver::gain_stm_normal_duty_body(
                &self.gains[(self.sent - 1) / 2],
                is_last_frame,
                tx,
            )?;
        }
        if self.mode == Mode::PhaseDutyFull {
            self.next_duty = !self.next_duty;
        }

        self.sent += 1;

        Ok(())
    }

    fn is_finished(&self) -> bool {
        if self.mode == Mode::PhaseDutyFull {
            self.sent == self.gains.len() * 2 + 1
        } else {
            self.sent == self.gains.len() + 1
        }
    }
}

impl DatagramBody<NormalPhaseTransducer> for GainSTM<NormalPhaseTransducer> {
    fn init(&mut self) -> Result<()> {
        self.sent = 0;
        self.next_duty = false;
        Ok(())
    }

    fn pack(
        &mut self,
        _geometry: &Geometry<NormalPhaseTransducer>,
        tx: &mut TxDatagram,
    ) -> Result<()> {
        autd3_driver::gain_stm_normal_head(tx);

        if DatagramBody::<NormalPhaseTransducer>::is_finished(self) {
            return Ok(());
        }

        let is_first_frame = self.sent == 0;
        let is_last_frame = self.sent + 1 == self.gains.len() + 1;

        if is_first_frame {
            autd3_driver::gain_stm_normal_phase_body(
                &[],
                is_first_frame,
                self.sample_freq_div,
                Mode::PhaseFull,
                is_last_frame,
                tx,
            )?;
            self.sent += 1;
            return Ok(());
        }

        let idx = self.sent - 1;
        autd3_driver::gain_stm_normal_phase_body(
            &self.gains[idx],
            is_first_frame,
            self.sample_freq_div,
            self.mode,
            is_last_frame,
            tx,
        )?;

        self.sent += 1;

        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.sent == self.gains.len() + 1
    }
}

impl<T: Transducer> STM for GainSTM<T> {
    fn set_freq(&mut self, freq: f64) -> f64 {
        let sample_freq = self.size() as f64 * freq;
        let div = ((FPGA_CLK_FREQ as f64 / sample_freq) as u32)
            .clamp(STM_SAMPLING_FREQ_DIV_MIN, u32::MAX);
        self.sample_freq_div = div;
        STM::freq(self)
    }

    fn freq(&self) -> f64 {
        STM::sampling_freq(self) / self.size() as f64
    }

    fn sampling_freq(&self) -> f64 {
        FPGA_CLK_FREQ as f64 / self.sample_freq_div as f64
    }

    fn set_sampling_freq_div(&mut self, freq_div: u32) {
        self.sample_freq_div = freq_div;
    }

    fn sampling_freq_div(&mut self) -> u32 {
        self.sample_freq_div
    }
}

impl Sendable<LegacyTransducer> for GainSTM<LegacyTransducer> {
    type H = Empty;
    type B = Filled;

    fn init(&mut self) -> Result<()> {
        DatagramBody::<LegacyTransducer>::init(self)
    }

    fn pack(
        &mut self,
        _msg_id: u8,
        geometry: &Geometry<LegacyTransducer>,
        tx: &mut TxDatagram,
    ) -> Result<()> {
        DatagramBody::<LegacyTransducer>::pack(self, geometry, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<LegacyTransducer>::is_finished(self)
    }
}

impl Sendable<NormalTransducer> for GainSTM<NormalTransducer> {
    type H = Empty;
    type B = Filled;

    fn init(&mut self) -> Result<()> {
        DatagramBody::<NormalTransducer>::init(self)
    }

    fn pack(
        &mut self,
        _msg_id: u8,
        geometry: &Geometry<NormalTransducer>,
        tx: &mut TxDatagram,
    ) -> Result<()> {
        DatagramBody::<NormalTransducer>::pack(self, geometry, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<NormalTransducer>::is_finished(self)
    }
}

impl Sendable<NormalPhaseTransducer> for GainSTM<NormalPhaseTransducer> {
    type H = Empty;
    type B = Filled;

    fn init(&mut self) -> Result<()> {
        DatagramBody::<NormalPhaseTransducer>::init(self)
    }

    fn pack(
        &mut self,
        _msg_id: u8,
        geometry: &Geometry<NormalPhaseTransducer>,
        tx: &mut TxDatagram,
    ) -> Result<()> {
        DatagramBody::<NormalPhaseTransducer>::pack(self, geometry, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<NormalPhaseTransducer>::is_finished(self)
    }
}
