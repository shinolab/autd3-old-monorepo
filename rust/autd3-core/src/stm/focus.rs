/*
 * File: point.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
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

pub struct Focus {
    point: Vector3,
    shift: u8,
}

impl Focus {
    pub fn new(point: Vector3) -> Self {
        Self { point, shift: 0 }
    }

    pub fn with_shift(point: Vector3, shift: u8) -> Self {
        Self { point, shift }
    }
}

pub struct FocusSTM {
    control_points: Vec<Focus>,
    sample_freq_div: u32,
    sent: usize,
    pub sound_speed: f64,
}

impl FocusSTM {
    pub fn new(sound_speed: f64) -> Self {
        Self {
            control_points: vec![],
            sample_freq_div: 4096,
            sent: 0,
            sound_speed,
        }
    }

    pub fn add(&mut self, point: Focus) -> Result<()> {
        self.control_points.push(point);
        Ok(())
    }

    pub fn control_points(&self) -> &[Focus] {
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
                *state += tr_num;
                Some(*state)
            })
            .map(|origin_idx| {
                let tr = &geometry[origin_idx];
                let origin = tr.position();
                let trans_inv =
                    Matrix3::from_columns(&[tr.x_direction(), tr.y_direction(), tr.z_direction()])
                        .transpose();
                src.iter()
                    .map(|p| {
                        let lp = trans_inv * (p.point - origin);
                        STMFocus::new(lp.x, lp.y, lp.z, p.shift)
                    })
                    .collect()
            })
            .collect();

        autd3_driver::focus_stm_body(
            &points,
            &mut self.sent,
            self.control_points.len(),
            self.sample_freq_div,
            self.sound_speed,
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
