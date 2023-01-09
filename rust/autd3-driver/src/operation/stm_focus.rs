/*
 * File: stm_focus.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{
    CPUControlFlags, DriverError, FPGAControlFlags, STMFocus, TxDatagram, FOCUS_STM_BUF_SIZE_MAX,
    FOCUS_STM_SAMPLING_FREQ_DIV_MIN,
};
use anyhow::Result;

#[derive(Default)]
pub struct FocusSTM {
    sent: usize,
    pub points: Vec<Vec<STMFocus>>,
    pub device_map: Vec<usize>,
    pub freq_div: u32,
    pub sound_speed: f64,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
}

impl FocusSTM {
    fn get_send_size(total_size: usize, sent: usize, device_map: &[usize]) -> usize {
        let tr_num = device_map.iter().min().unwrap();
        let data_len = tr_num * std::mem::size_of::<u16>();
        let max_size = if sent == 0 {
            (data_len
                - std::mem::size_of::<u16>()
                - std::mem::size_of::<u32>()
                - std::mem::size_of::<u32>()
                - std::mem::size_of::<u16>()
                - std::mem::size_of::<u16>())
                / std::mem::size_of::<STMFocus>()
        } else {
            (data_len - std::mem::size_of::<u16>()) / std::mem::size_of::<STMFocus>()
        };
        (total_size - sent).min(max_size)
    }
}

impl Operation for FocusSTM {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_END);

        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_MODE, true);
        tx.header_mut()
            .fpga_flag
            .remove(FPGAControlFlags::STM_GAIN_MODE);
        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        if self.points[0].len() > FOCUS_STM_BUF_SIZE_MAX {
            return Err(DriverError::FocusSTMPointSizeOutOfRange(self.points[0].len()).into());
        }

        if let Some(idx) = self.start_idx {
            if idx as usize >= self.points[0].len() {
                return Err(DriverError::STMStartIndexOutOfRange.into());
            }
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_START_IDX, true);
        } else {
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_START_IDX, false);
        }
        if let Some(idx) = self.finish_idx {
            if idx as usize >= self.points[0].len() {
                return Err(DriverError::STMFinishIndexOutOfRange.into());
            }
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_FINISH_IDX, true);
        } else {
            tx.header_mut()
                .fpga_flag
                .set(FPGAControlFlags::USE_FINISH_IDX, false);
        }

        let send_size = Self::get_send_size(self.points[0].len(), self.sent, &self.device_map);
        if self.sent == 0 {
            if self.freq_div < FOCUS_STM_SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::FocusSTMFreqDivOutOfRange(self.freq_div).into());
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            let sound_speed = (self.sound_speed / 1e3 * 1024.0).round() as u32;
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                let s = &self.points[idx];
                d.focus_stm_initial_mut().set_size(send_size as _);
                d.focus_stm_initial_mut().set_freq_div(self.freq_div);
                d.focus_stm_initial_mut().set_sound_speed(sound_speed);
                d.focus_stm_initial_mut()
                    .set_points(&s[self.sent..self.sent + send_size]);
                d.focus_stm_initial_mut()
                    .set_start_idx(self.start_idx.unwrap_or(0));
                d.focus_stm_initial_mut()
                    .set_finish_idx(self.finish_idx.unwrap_or(0));
            });
        } else {
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                let s = &self.points[idx];
                d.focus_stm_subsequent_mut().set_size(send_size as _);
                d.focus_stm_subsequent_mut()
                    .set_points(&s[self.sent..self.sent + send_size]);
            });
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        if self.sent + send_size == self.points[0].len() {
            tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
        }

        tx.num_bodies = tx.num_devices();
        self.sent += send_size;

        Ok(())
    }

    fn init(&mut self) {
        self.sent = 0;
    }

    fn is_finished(&self) -> bool {
        self.sent == self.points[0].len()
    }
}
