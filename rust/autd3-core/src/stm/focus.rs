/*
 * File: point.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    geometry::{Geometry, Transducer, Vector3},
    interface::{DatagramBody, Empty, Filled, Sendable},
};

use anyhow::{Ok, Result};
use autd3_driver::{
    SeqFocus, TxDatagram, FOCUS_STM_BODY_DATA_SIZE, FOCUS_STM_HEAD_DATA_SIZE, FPGA_CLK_FREQ,
    STM_SAMPLING_FREQ_DIV_MIN,
};

use super::STM;

pub struct FocusSTM {
    control_points: Vec<(Vector3, u8)>,
    sample_freq_div: u32,
    sent: usize,
}

impl FocusSTM {
    pub fn new() -> Self {
        Self::with_control_points(vec![])
    }

    pub fn with_control_points(control_points: Vec<(Vector3, u8)>) -> Self {
        Self {
            control_points,
            sample_freq_div: 4096,
            sent: 0,
        }
    }

    pub fn add(&mut self, point: Vector3, duty_shift: u8) -> Result<()> {
        if self.control_points.len() + 1 > autd3_driver::FOCUS_STM_BUF_SIZE_MAX {
            return Err(autd3_driver::FPGAError::FocusSTMOutOfBuffer(
                self.control_points.len() + 1,
            )
            .into());
        }
        self.control_points.push((point, duty_shift));
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.control_points.len()
    }

    pub fn control_points(&self) -> &[(Vector3, u8)] {
        &self.control_points
    }
}

impl Default for FocusSTM {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transducer> DatagramBody<T> for FocusSTM {
    fn init(&mut self) -> Result<()> {
        self.sent = 0;
        Ok(())
    }

    fn pack(&mut self, geometry: &Geometry<T>, tx: &mut TxDatagram) -> Result<()> {
        autd3_driver::focus_stm_initial(tx);

        if DatagramBody::<T>::is_finished(self) {
            return Ok(());
        }

        let is_first_frame = self.sent == 0;
        let max_size = if is_first_frame {
            FOCUS_STM_HEAD_DATA_SIZE
        } else {
            FOCUS_STM_BODY_DATA_SIZE
        };
        let send_size = (self.control_points.len() - self.sent).min(max_size);
        let is_last_frame = self.sent + send_size == self.control_points.len();

        let points: Vec<Vec<_>> = geometry
            .devices()
            .iter()
            .map(|dev| {
                self.control_points()[self.sent..(self.sent + send_size)]
                    .iter()
                    .map(|(p, d)| {
                        let lp = dev.local_position(p);
                        SeqFocus::new(lp.x, lp.y, lp.z, *d)
                    })
                    .collect()
            })
            .collect();

        autd3_driver::focus_stm_body(
            &points,
            is_first_frame,
            self.sample_freq_div,
            geometry.sound_speed(),
            is_last_frame,
            tx,
        )?;

        self.sent += send_size;

        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.sent == self.control_points.len()
    }
}

impl<T: Transducer> Sendable<T> for FocusSTM {
    type H = Empty;
    type B = Filled;

    fn init(&mut self) -> Result<()> {
        DatagramBody::<T>::init(self)
    }

    fn pack(&mut self, _msg_id: u8, geometry: &Geometry<T>, tx: &mut TxDatagram) -> Result<()> {
        DatagramBody::<T>::pack(self, geometry, tx)
    }

    fn is_finished(&self) -> bool {
        DatagramBody::<T>::is_finished(self)
    }
}

impl STM for FocusSTM {
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
