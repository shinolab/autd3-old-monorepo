/*
 * File: stm_focus.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{
    float, CPUControlFlags, DriverError, FPGAControlFlags, STMFocus, TxDatagram,
    FOCUS_STM_BODY_INITIAL_SIZE, FOCUS_STM_BODY_SUBSEQUENT_SIZE, FOCUS_STM_BUF_SIZE_MAX,
    FPGA_SUB_CLK_FREQ_DIV, METER, SAMPLING_FREQ_DIV_MIN,
};

#[derive(Default, Clone, Copy)]
pub struct FocusSTMProps {
    pub freq_div: u32,
    pub sound_speed: float,
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
    pub const fn new(points: Vec<Vec<STMFocus>>, tr_num_min: usize, props: FocusSTMProps) -> Self {
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
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
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

        if self.points[0].len() < 2 || self.points[0].len() > FOCUS_STM_BUF_SIZE_MAX {
            return Err(DriverError::FocusSTMPointSizeOutOfRange(
                self.points[0].len(),
            ));
        }

        if let Some(idx) = self.props.start_idx {
            if idx as usize >= self.points[0].len() {
                return Err(DriverError::STMStartIndexOutOfRange);
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
                return Err(DriverError::STMFinishIndexOutOfRange);
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
            let freq_div = self.props.freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;
            if freq_div < SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::FocusSTMFreqDivOutOfRange(freq_div));
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            let sound_speed = (self.props.sound_speed / METER * 1024.0).round() as u32;
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                let s = &self.points[idx];
                d.focus_stm_initial_mut().set_size(send_size as _);
                d.focus_stm_initial_mut().set_freq_div(freq_div);
                d.focus_stm_initial_mut().set_sound_speed(sound_speed);
                d.focus_stm_initial_mut().set_points(&s[..send_size]);
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

#[cfg(test)]
mod test {
    use rand::prelude::*;

    use super::*;

    const NUM_TRANS_IN_UNIT: usize = 249;

    #[test]
    fn stm_focus() {
        let device_map = [
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
        ];
        let mut tx = TxDatagram::new(&device_map);

        let mut rng = rand::thread_rng();

        let size = 150;
        let p = (0..size)
            .map(|_| {
                STMFocus::new(
                    rng.gen_range(-1000.0..1000.0),
                    rng.gen_range(-1000.0..1000.0),
                    rng.gen_range(-1000.0..1000.0),
                    rng.gen_range(0..0xFF),
                )
            })
            .collect::<Vec<_>>();
        let points = vec![p; tx.num_devices()];

        let sound_speed = 340e3;
        let sp = (sound_speed / 1e3) as u32 * 1024;

        let props = FocusSTMProps {
            freq_div: 512,
            sound_speed,
            start_idx: Some(1),
            finish_idx: Some(2),
        };

        let mut op = FocusSTM::new(points.clone(), *device_map.iter().min().unwrap(), props);

        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::STM_GAIN_MODE));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_START_IDX));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_FINISH_IDX));
        (0..10).for_each(|i| {
            let stm = tx.body(i).focus_stm_initial();
            assert_eq!(stm.data[0], 60);
            assert_eq!((stm.data[2] as u32) << 16 | stm.data[1] as u32, 4096);
            assert_eq!((stm.data[4] as u32) << 16 | stm.data[3] as u32, sp);
            assert_eq!(stm.data[5], 1);
            assert_eq!(stm.data[6], 2);
        });
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::STM_GAIN_MODE));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_START_IDX));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_FINISH_IDX));
        (0..10).for_each(|i| {
            let stm = tx.body(i).focus_stm_subsequent();
            assert_eq!(stm.data[0], 62);
        });
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::STM_GAIN_MODE));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_START_IDX));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_FINISH_IDX));
        (0..10).for_each(|i| {
            let stm = tx.body(i).focus_stm_subsequent();
            assert_eq!(stm.data[0], 28);
        });
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::STM_GAIN_MODE));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_START_IDX));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_FINISH_IDX));
        assert_eq!(tx.num_bodies, 0);

        op.init();
        assert!(!op.is_finished());

        let props = FocusSTMProps {
            start_idx: None,
            ..props
        };

        let mut op = FocusSTM::new(points.clone(), *device_map.iter().min().unwrap(), props);
        op.init();
        op.pack(&mut tx).unwrap();
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_START_IDX));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_FINISH_IDX));

        let props = FocusSTMProps {
            finish_idx: None,
            ..props
        };

        let mut op = FocusSTM::new(points.clone(), *device_map.iter().min().unwrap(), props);
        op.init();
        op.pack(&mut tx).unwrap();
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_START_IDX));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::USE_FINISH_IDX));

        let props = FocusSTMProps {
            start_idx: Some(size),
            ..props
        };
        let mut op = FocusSTM::new(points.clone(), *device_map.iter().min().unwrap(), props);
        op.init();
        assert!(op.pack(&mut tx).is_err());

        let props = FocusSTMProps {
            start_idx: None,
            finish_idx: Some(size),
            ..props
        };
        let mut op = FocusSTM::new(points.clone(), *device_map.iter().min().unwrap(), props);
        op.init();
        assert!(op.pack(&mut tx).is_err());

        let props = FocusSTMProps {
            freq_div: 511,
            start_idx: None,
            finish_idx: None,
            ..props
        };
        let mut op = FocusSTM::new(points, *device_map.iter().min().unwrap(), props);
        op.init();
        assert!(op.pack(&mut tx).is_err());
    }
}
