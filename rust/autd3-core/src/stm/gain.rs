/*
 * File: gain.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::marker::PhantomData;

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    gain::Gain,
    geometry::{Geometry, LegacyTransducer, NormalPhaseTransducer, NormalTransducer, Transducer},
};

use anyhow::{Ok, Result};
use autd3_driver::{Drive, Mode, TxDatagram};

use super::STM;

pub struct GainSTM<T: Transducer> {
    gains: Vec<Vec<Drive>>,
    sample_freq_div: u32,
    next_duty: bool,
    sent: usize,
    mode: Mode,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
    phantom: PhantomData<T>,
}

impl<T: Transducer> GainSTM<T> {
    pub fn new() -> Self {
        Self {
            gains: vec![],
            sample_freq_div: 4096,
            next_duty: false,
            sent: 0,
            mode: Mode::PhaseDutyFull,
            start_idx: None,
            finish_idx: None,
            phantom: PhantomData,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn add<G: Gain<T>>(&mut self, gain: G, geometry: &Geometry<T>) -> Result<()> {
        let mut gain = gain;
        gain.build(geometry)?;
        let drives = gain.take_drives();
        self.gains.push(drives);
        Ok(())
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
        autd3_driver::gain_stm_legacy_header(tx);
        if DatagramBody::<LegacyTransducer>::is_finished(self) {
            return Ok(());
        }
        autd3_driver::gain_stm_legacy_body(
            &self.gains,
            &mut self.sent,
            self.sample_freq_div,
            self.mode,
            self.start_idx,
            self.finish_idx,
            tx,
        )
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
        autd3_driver::gain_stm_normal_header(tx);
        if DatagramBody::<NormalTransducer>::is_finished(self) {
            return Ok(());
        }

        if self.sent == 0 {
            self.sent += 1;
            return autd3_driver::gain_stm_normal_phase_body(
                &self.gains,
                0,
                self.sample_freq_div,
                self.mode,
                self.start_idx,
                self.finish_idx,
                tx,
            );
        }

        match self.mode {
            Mode::PhaseDutyFull => {
                if self.next_duty {
                    self.next_duty = !self.next_duty;
                    autd3_driver::gain_stm_normal_duty_body(&self.gains, self.sent, tx)
                } else {
                    self.next_duty = !self.next_duty;
                    let sent = self.sent;
                    self.sent += 1;
                    autd3_driver::gain_stm_normal_phase_body(
                        &self.gains,
                        sent,
                        self.sample_freq_div,
                        self.mode,
                        self.start_idx,
                        self.finish_idx,
                        tx,
                    )
                }
            }
            Mode::PhaseFull => {
                let sent = self.sent;
                self.sent += 1;
                autd3_driver::gain_stm_normal_phase_body(
                    &self.gains,
                    sent,
                    self.sample_freq_div,
                    self.mode,
                    self.start_idx,
                    self.finish_idx,
                    tx,
                )
            }
            Mode::PhaseHalf => Err(autd3_driver::DriverError::PhaseHalfNotSupported.into()),
        }
    }

    fn is_finished(&self) -> bool {
        self.sent == self.gains.len() + 1
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
        autd3_driver::gain_stm_normal_header(tx);
        if DatagramBody::<NormalPhaseTransducer>::is_finished(self) {
            return Ok(());
        }
        let sent = self.sent;
        self.sent += 1;
        autd3_driver::gain_stm_normal_phase_body(
            &self.gains,
            sent,
            self.sample_freq_div,
            self.mode,
            self.start_idx,
            self.finish_idx,
            tx,
        )
    }

    fn is_finished(&self) -> bool {
        self.sent == self.gains.len() + 1
    }
}

impl<T: Transducer> STM for GainSTM<T> {
    fn size(&self) -> usize {
        self.gains.len()
    }

    fn set_sampling_freq_div(&mut self, freq_div: u32) {
        self.sample_freq_div = freq_div;
    }

    fn sampling_freq_div(&self) -> u32 {
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
