/*
 * File: stm_gain.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{
    CPUControlFlags, Drive, DriverError, FPGAControlFlags, Mode, TxDatagram, GAIN_STM_BUF_SIZE_MAX,
    GAIN_STM_LEGACY_BUF_SIZE_MAX, GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN,
    GAIN_STM_SAMPLING_FREQ_DIV_MIN,
};

#[derive(Default, Clone, Copy)]
pub struct GainSTMProps {
    pub freq_div: u32,
    pub mode: Mode,
    pub start_idx: Option<u16>,
    pub finish_idx: Option<u16>,
}

pub struct GainSTMLegacy {
    sent: usize,
    drives: Vec<Vec<Drive>>,
    props: GainSTMProps,
}

impl GainSTMLegacy {
    pub fn new(drives: Vec<Vec<Drive>>, props: GainSTMProps) -> Self {
        Self {
            sent: 0,
            drives,
            props,
        }
    }
}

impl Operation for GainSTMLegacy {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_END);

        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::LEGACY_MODE, true);
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_MODE, true);
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_GAIN_MODE, true);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        if self.drives.len() > GAIN_STM_LEGACY_BUF_SIZE_MAX {
            return Err(DriverError::GainSTMLegacySizeOutOfRange(self.drives.len()));
        }

        let mut is_last_frame = false;

        if let Some(idx) = self.props.start_idx {
            if idx as usize >= self.drives.len() {
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
            if idx as usize >= self.drives.len() {
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

        if self.sent == 0 {
            if self.props.freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::GainSTMLegacyFreqDivOutOfRange(
                    self.props.freq_div,
                ));
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                d.gain_stm_initial_mut().set_freq_div(self.props.freq_div);
                d.gain_stm_initial_mut().set_mode(self.props.mode);
                d.gain_stm_initial_mut().set_cycle(self.drives.len());
                d.gain_stm_initial_mut()
                    .set_start_idx(self.props.start_idx.unwrap_or(0));
                d.gain_stm_initial_mut()
                    .set_finish_idx(self.props.finish_idx.unwrap_or(0));
            });
            self.sent += 1;
        } else {
            match self.props.mode {
                Mode::PhaseDutyFull => {
                    is_last_frame = self.sent + 1 >= self.drives.len() + 1;
                    tx.legacy_drives_mut()
                        .iter_mut()
                        .zip(&self.drives[self.sent - 1])
                        .for_each(|(d, s)| d.set(s));
                    self.sent += 1;
                }
                Mode::PhaseFull => {
                    is_last_frame = self.sent + 2 >= self.drives.len() + 1;
                    tx.legacy_phase_full_mut::<0>()
                        .iter_mut()
                        .zip(&self.drives[self.sent - 1])
                        .for_each(|(d, s)| d.set(s));
                    self.sent += 1;
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_full_mut::<1>()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(s));
                        self.sent += 1;
                    }
                }
                Mode::PhaseHalf => {
                    is_last_frame = self.sent + 4 >= self.drives.len() + 1;
                    tx.legacy_phase_half_mut::<0>()
                        .iter_mut()
                        .zip(&self.drives[self.sent - 1])
                        .for_each(|(d, s)| d.set(s));
                    self.sent += 1;
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut::<1>()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(s));
                        self.sent += 1;
                    }
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut::<2>()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(s));
                        self.sent += 1;
                    }
                    if self.sent - 1 < self.drives.len() {
                        tx.legacy_phase_half_mut::<3>()
                            .iter_mut()
                            .zip(&self.drives[self.sent - 1])
                            .for_each(|(d, s)| d.set(s));
                        self.sent += 1;
                    }
                }
            }
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        if is_last_frame {
            tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
        }

        tx.num_bodies = tx.num_devices();

        Ok(())
    }

    fn init(&mut self) {
        self.sent = 0;
    }

    fn is_finished(&self) -> bool {
        self.sent > self.drives.len()
    }
}

#[derive(Default)]
pub struct GainSTMAdvanced {
    sent: usize,
    next_duty: bool,
    drives: Vec<Vec<Drive>>,
    cycles: Vec<u16>,
    props: GainSTMProps,
}

impl GainSTMAdvanced {
    pub fn new(drives: Vec<Vec<Drive>>, cycles: Vec<u16>, props: GainSTMProps) -> Self {
        Self {
            sent: 0,
            next_duty: false,
            drives,
            cycles,
            props,
        }
    }

    fn pack_phase(&self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        if self.drives.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(DriverError::GainSTMSizeOutOfRange(self.drives.len()));
        }

        tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

        if let Some(idx) = self.props.start_idx {
            if idx as usize >= self.drives.len() {
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
            if idx as usize >= self.drives.len() {
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

        if self.sent == 0 {
            if self.props.freq_div < GAIN_STM_SAMPLING_FREQ_DIV_MIN {
                return Err(DriverError::GainSTMFreqDivOutOfRange(self.props.freq_div));
            }
            tx.header_mut()
                .cpu_flag
                .set(CPUControlFlags::STM_BEGIN, true);
            (0..tx.num_devices()).for_each(|idx| {
                let d = tx.body_mut(idx);
                d.gain_stm_initial_mut().set_freq_div(self.props.freq_div);
                d.gain_stm_initial_mut().set_mode(self.props.mode);
                d.gain_stm_initial_mut().set_cycle(self.drives.len());
                d.gain_stm_initial_mut()
                    .set_start_idx(self.props.start_idx.unwrap_or(0));
                d.gain_stm_initial_mut()
                    .set_finish_idx(self.props.finish_idx.unwrap_or(0));
            });
        } else {
            tx.phases_mut()
                .iter_mut()
                .zip(self.drives[self.sent - 1].iter())
                .zip(self.cycles.iter())
                .for_each(|((d, s), &c)| d.set(s, c));
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        if self.sent + 1 == self.drives.len() + 1 {
            tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
        }

        tx.num_bodies = tx.num_devices();

        Ok(())
    }

    fn pack_duty(&self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        if self.drives.len() > GAIN_STM_BUF_SIZE_MAX {
            return Err(DriverError::GainSTMSizeOutOfRange(self.drives.len()));
        }

        tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

        tx.duties_mut()
            .iter_mut()
            .zip(self.drives[self.sent - 1].iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        if self.sent + 1 == self.drives.len() + 1 {
            tx.header_mut().cpu_flag.set(CPUControlFlags::STM_END, true);
        }

        tx.num_bodies = tx.num_devices();

        Ok(())
    }
}

impl Operation for GainSTMAdvanced {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::STM_END);

        tx.header_mut()
            .fpga_flag
            .remove(FPGAControlFlags::LEGACY_MODE);
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_MODE, true);
        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::STM_GAIN_MODE, true);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        if self.sent == 0 {
            self.pack_phase(tx)?;
            self.sent += 1;
            return Ok(());
        }

        match self.props.mode {
            Mode::PhaseDutyFull => {
                if self.next_duty {
                    self.pack_duty(tx)?;
                    self.sent += 1;
                } else {
                    self.pack_phase(tx)?;
                }
                self.next_duty = !self.next_duty;
            }
            Mode::PhaseFull => {
                self.pack_phase(tx)?;
                self.sent += 1;
            }
            Mode::PhaseHalf => return Err(DriverError::PhaseHalfNotSupported),
        }

        Ok(())
    }

    fn init(&mut self) {
        self.sent = 0;
        self.next_duty = false;
    }

    fn is_finished(&self) -> bool {
        self.sent > self.drives.len()
    }
}

#[cfg(test)]
mod test {
    use rand::prelude::*;

    use crate::{AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive};

    use super::*;

    const NUM_TRANS_IN_UNIT: usize = 249;

    #[test]
    fn gain_stm_legacy() {
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
        let d = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| Drive {
                phase: rng.gen_range(0.0..1.0),
                amp: rng.gen_range(0.0..1.0),
            })
            .collect::<Vec<_>>();
        let drives = vec![d; 2];

        let props = GainSTMProps {
            freq_div: 152,
            mode: Mode::PhaseDutyFull,
            start_idx: Some(1),
            finish_idx: Some(1),
        };

        let mut op = GainSTMLegacy::new(drives.clone(), props);

        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for i in 0..10 {
            let stm = tx.body(i).gain_stm_initial();
            assert_eq!((stm.data[1] as u32) << 16 | stm.data[0] as u32, 152);
            assert_eq!(stm.data[2], Mode::PhaseDutyFull as u16);
            assert_eq!(stm.data[3], 2);
            assert_eq!(stm.data[4], 1);
            assert_eq!(stm.data[5], 1);
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for i in 0..10 {
            assert_eq!(
                (tx.body_raw_mut()[i] & 0xFF) as u8,
                LegacyDrive::to_phase(&drives[0][i])
            );
            assert_eq!(
                (tx.body_raw_mut()[i] >> 8) as u8,
                LegacyDrive::to_duty(&drives[0][i])
            );
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for i in 0..10 {
            assert_eq!(
                (tx.body_raw_mut()[i] & 0xFF) as u8,
                LegacyDrive::to_phase(&drives[1][i])
            );
            assert_eq!(
                (tx.body_raw_mut()[i] >> 8) as u8,
                LegacyDrive::to_duty(&drives[1][i])
            );
        }
        assert_eq!(tx.num_bodies, 10);

        op.init();
        assert!(!op.is_finished());

        let props = GainSTMProps {
            start_idx: None,
            ..props
        };

        let mut op = GainSTMLegacy::new(drives.clone(), props);
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

        let props = GainSTMProps {
            finish_idx: None,
            ..props
        };

        let mut op = GainSTMLegacy::new(drives.clone(), props);
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

        let props = GainSTMProps {
            start_idx: Some(2),
            ..props
        };
        let mut op = GainSTMLegacy::new(drives.clone(), props);
        op.init();
        assert!(op.pack(&mut tx).is_err());

        let props = GainSTMProps {
            start_idx: None,
            finish_idx: Some(2),
            ..props
        };
        let mut op = GainSTMLegacy::new(drives, props);
        op.init();
        assert!(op.pack(&mut tx).is_err());
    }

    #[test]
    fn gain_stm_advanced() {
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
        let d = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| Drive {
                phase: rng.gen_range(0.0..1.0),
                amp: rng.gen_range(0.0..1.0),
            })
            .collect::<Vec<_>>();
        let drives = vec![d; 2];

        let cycles = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| rng.gen_range(2..0xFFFF))
            .collect::<Vec<_>>();

        let props = GainSTMProps {
            freq_div: 276,
            mode: Mode::PhaseDutyFull,
            start_idx: Some(1),
            finish_idx: Some(1),
        };

        let mut op = GainSTMAdvanced::new(drives.clone(), cycles.clone(), props);

        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for i in 0..10 {
            let stm = tx.body(i).gain_stm_initial();
            assert_eq!((stm.data[1] as u32) << 16 | stm.data[0] as u32, 276);
            assert_eq!(stm.data[2], Mode::PhaseDutyFull as u16);
            assert_eq!(stm.data[3], 2);
            assert_eq!(stm.data[4], 1);
            assert_eq!(stm.data[5], 1);
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for ((&d, drive), &cycle) in tx
            .body_raw_mut()
            .iter()
            .zip(drives[0].iter())
            .zip(cycles.iter())
        {
            assert_eq!(d, AdvancedDrivePhase::to_phase(drive, cycle))
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for ((&d, drive), &cycle) in tx
            .body_raw_mut()
            .iter()
            .zip(drives[0].iter())
            .zip(cycles.iter())
        {
            assert_eq!(d, AdvancedDriveDuty::to_duty(drive, cycle))
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for ((&d, drive), &cycle) in tx
            .body_raw_mut()
            .iter()
            .zip(drives[1].iter())
            .zip(cycles.iter())
        {
            assert_eq!(d, AdvancedDrivePhase::to_phase(drive, cycle))
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for ((&d, drive), &cycle) in tx
            .body_raw_mut()
            .iter()
            .zip(drives[1].iter())
            .zip(cycles.iter())
        {
            assert_eq!(d, AdvancedDriveDuty::to_duty(drive, cycle))
        }
        assert_eq!(tx.num_bodies, 10);

        op.init();
        assert!(!op.is_finished());

        let props = GainSTMProps {
            start_idx: None,
            ..props
        };

        let mut op = GainSTMLegacy::new(drives.clone(), props);
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

        let props = GainSTMProps {
            finish_idx: None,
            ..props
        };

        let mut op = GainSTMLegacy::new(drives.clone(), props);
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

        let props = GainSTMProps {
            start_idx: Some(2),
            ..props
        };
        let mut op = GainSTMLegacy::new(drives.clone(), props);
        op.init();
        assert!(op.pack(&mut tx).is_err());

        let props = GainSTMProps {
            start_idx: None,
            finish_idx: Some(2),
            ..props
        };
        let mut op = GainSTMLegacy::new(drives, props);
        op.init();
        assert!(op.pack(&mut tx).is_err());
    }

    #[test]
    fn gain_stm_advanced_phase() {
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
        let d = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| Drive {
                phase: rng.gen_range(0.0..1.0),
                amp: rng.gen_range(0.0..1.0),
            })
            .collect::<Vec<_>>();
        let drives = vec![d; 2];

        let cycles = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| rng.gen_range(2..0xFFFF))
            .collect::<Vec<_>>();

        let props = GainSTMProps {
            freq_div: 276,
            mode: Mode::PhaseFull,
            start_idx: Some(1),
            finish_idx: Some(1),
        };

        let mut op = GainSTMAdvanced::new(drives.clone(), cycles.clone(), props);

        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for i in 0..10 {
            let stm = tx.body(i).gain_stm_initial();
            assert_eq!((stm.data[1] as u32) << 16 | stm.data[0] as u32, 276);
            assert_eq!(stm.data[2], Mode::PhaseFull as u16);
            assert_eq!(stm.data[3], 2);
            assert_eq!(stm.data[4], 1);
            assert_eq!(stm.data[5], 1);
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for ((&d, drive), &cycle) in tx
            .body_raw_mut()
            .iter()
            .zip(drives[0].iter())
            .zip(cycles.iter())
        {
            assert_eq!(d, AdvancedDrivePhase::to_phase(drive, cycle))
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::STM_END));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert!(tx
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
        for ((&d, drive), &cycle) in tx
            .body_raw_mut()
            .iter()
            .zip(drives[1].iter())
            .zip(cycles.iter())
        {
            assert_eq!(d, AdvancedDrivePhase::to_phase(drive, cycle))
        }
        assert_eq!(tx.num_bodies, 10);

        op.init();
        assert!(!op.is_finished());

        let props = GainSTMProps {
            start_idx: None,
            ..props
        };

        let mut op = GainSTMLegacy::new(drives.clone(), props);
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

        let props = GainSTMProps {
            finish_idx: None,
            ..props
        };

        let mut op = GainSTMLegacy::new(drives.clone(), props);
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

        let props = GainSTMProps {
            start_idx: Some(2),
            ..props
        };
        let mut op = GainSTMLegacy::new(drives.clone(), props);
        op.init();
        assert!(op.pack(&mut tx).is_err());

        let props = GainSTMProps {
            start_idx: None,
            finish_idx: Some(2),
            ..props
        };
        let mut op = GainSTMLegacy::new(drives, props);
        op.init();
        assert!(op.pack(&mut tx).is_err());
    }
}
