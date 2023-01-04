/*
 * File: point.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{Geometry, Matrix3, Transducer, Vector3},
};

use anyhow::{Ok, Result};
use autd3_driver::*;

use super::STM;

pub struct FocusSTM {
    control_points: Vec<(Vector3, u8)>,
    sample_freq_div: u32,
    sent: usize,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
    pub sound_speed: Option<f64>,
}

impl FocusSTM {
    pub fn new() -> Self {
        Self {
            control_points: vec![],
            sample_freq_div: 4096,
            sent: 0,
            start_idx: None,
            finish_idx: None,
            sound_speed: None,
        }
    }

    pub fn add(&mut self, point: Vector3) -> Result<()> {
        self.control_points.push((point, 0));
        Ok(())
    }

    pub fn add_with_shift(&mut self, point: Vector3, shift: u8) -> Result<()> {
        self.control_points.push((point, shift));
        Ok(())
    }

    pub fn control_points(&self) -> &[(Vector3, u8)] {
        &self.control_points
    }
}

impl<T: Transducer> DatagramBody<T> for FocusSTM {
    fn init(&mut self) -> Result<()> {
        self.sent = 0;
        Ok(())
    }

    fn pack(&mut self, geometry: &Geometry<T>, tx: &mut TxDatagram) -> Result<()> {
        autd3_driver::focus_stm_header(tx);

        if DatagramBody::<T>::is_finished(self) {
            return Ok(());
        }

        let send_size = autd3_driver::focus_stm_send_size(
            self.control_points.len(),
            self.sent,
            geometry.device_map(),
        );

        let src = &self.control_points[self.sent..(self.sent + send_size)];
        let points: Vec<Vec<_>> = geometry
            .device_map()
            .iter()
            .scan(0, |state, tr_num| {
                let r = Some(*state);
                *state += tr_num;
                r
            })
            .map(|origin_idx| {
                let tr = &geometry[origin_idx];
                let origin = tr.position();
                let trans_inv =
                    Matrix3::from_columns(&[tr.x_direction(), tr.y_direction(), tr.z_direction()])
                        .transpose();
                src.iter()
                    .map(|(p, shift)| {
                        let lp = trans_inv * (p - origin);
                        STMFocus::new(lp.x, lp.y, lp.z, *shift)
                    })
                    .collect()
            })
            .collect();

        let sound_speed = self.sound_speed.unwrap_or(geometry.sound_speed);
        autd3_driver::focus_stm_body(
            &points,
            &mut self.sent,
            self.control_points.len(),
            self.sample_freq_div,
            sound_speed,
            self.start_idx,
            self.finish_idx,
            tx,
        )
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
    fn size(&self) -> usize {
        self.control_points.len()
    }

    fn set_sampling_freq_div(&mut self, freq_div: u32) {
        self.sample_freq_div = freq_div;
    }

    fn sampling_freq_div(&self) -> u32 {
        self.sample_freq_div
    }
}

impl Default for FocusSTM {
    fn default() -> Self {
        Self::new()
    }
}
