/*
 * File: stm_focus.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{
    CPUControlFlags, DriverError, FPGAControlFlags, STMFocus, TxDatagram,
    FOCUS_STM_BODY_INITIAL_SIZE, FOCUS_STM_BODY_SUBSEQUENT_SIZE, FOCUS_STM_BUF_SIZE_MAX,
    FOCUS_STM_SAMPLING_FREQ_DIV_MIN,
};
use anyhow::Result;

#[derive(Default, Clone, Copy)]
pub struct FocusSTMProps {
    pub freq_div: u32,
    pub sound_speed: f64,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
}

pub struct FocusSTM {
    sent: usize,
    points: Vec<Vec<STMFocus>>,
    tr_num_min: usize,
    props: FocusSTMProps,
}

impl FocusSTM {
    pub fn new(points: Vec<Vec<STMFocus>>, tr_num_min: usize, props: FocusSTMProps) -> Self {
        Self {
            sent: 0,
            points,
            tr_num_min,
            props,
        }
    }
}

impl FocusSTM {
    fn get_send_size(total_size: usize, sent: usize, tr_num_min: usize) -> usize {
        let data_len = tr_num_min * std::mem::size_of::<u16>();
        let max_size = if sent == 0 {
            (data_len - FOCUS_STM_BODY_INITIAL_SIZE) / std::mem::size_of::<STMFocus>()
        } else {
            (data_len - FOCUS_STM_BODY_SUBSEQUENT_SIZE) / std::mem::size_of::<STMFocus>()
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

        if let Some(idx) = self.props.start_idx {
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
        if let Some(idx) = self.props.finish_idx {
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

        let send_size = Self::get_send_size(self.points[0].len(), self.sent, self.tr_num_min);
        if self.sent == 0 {
            if self.props.freq_div < FOCUS_STM_SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::FocusSTMFreqDivOutOfRange(self.props.freq_div).into());
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            let sound_speed = (self.props.sound_speed / 1e3 * 1024.0).round() as u32;
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                let s = &self.points[idx];
                d.focus_stm_initial_mut().set_size(send_size as _);
                d.focus_stm_initial_mut().set_freq_div(self.props.freq_div);
                d.focus_stm_initial_mut().set_sound_speed(sound_speed);
                d.focus_stm_initial_mut()
                    .set_points(&s[self.sent..self.sent + send_size]);
                d.focus_stm_initial_mut()
                    .set_start_idx(self.props.start_idx.unwrap_or(0));
                d.focus_stm_initial_mut()
                    .set_finish_idx(self.props.finish_idx.unwrap_or(0));
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
