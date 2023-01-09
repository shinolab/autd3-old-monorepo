/*
 * File: point.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
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
    op: autd3_driver::FocusSTM,
}

impl FocusSTM {
    pub fn new() -> Self {
        Self {
            control_points: vec![],
            op: Default::default(),
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
    fn init(&mut self, geometry: &Geometry<T>) -> Result<()> {
        self.op.init();
        self.op.sound_speed = geometry.sound_speed;
        self.op.device_map = geometry.device_map().to_vec();
        self.op.points = geometry
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
                self.control_points
                    .iter()
                    .map(|(p, shift)| {
                        let lp = trans_inv * (p - origin);
                        STMFocus::new(lp.x, lp.y, lp.z, *shift)
                    })
                    .collect()
            })
            .collect();

        Ok(())
    }

    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<T: Transducer> Sendable<T> for FocusSTM {
    type H = Empty;
    type B = Filled;

    fn init(&mut self, geometry: &Geometry<T>) -> Result<()> {
        DatagramBody::<T>::init(self, geometry)
    }

    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        DatagramBody::<T>::pack(self, tx)
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
        self.op.freq_div = freq_div;
    }

    fn sampling_freq_div(&self) -> u32 {
        self.op.freq_div
    }

    fn set_start_idx(&mut self, idx: Option<u16>) {
        self.op.start_idx = idx;
    }

    fn start_idx(&self) -> Option<u16> {
        self.op.start_idx
    }

    fn set_finish_idx(&mut self, idx: Option<u16>) {
        self.op.finish_idx = idx;
    }

    fn finish_idx(&self) -> Option<u16> {
        self.op.finish_idx
    }
}

impl Default for FocusSTM {
    fn default() -> Self {
        Self::new()
    }
}
