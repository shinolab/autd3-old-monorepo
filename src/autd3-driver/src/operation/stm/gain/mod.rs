/*
 * File: mod.rs
 * Project: gain
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod gain_stm_control_flags;
mod gain_stm_mode;
mod reduced_phase;

use std::collections::HashMap;

use crate::{
    derive::prelude::{AUTDInternalError, Drive, Gain, GainFilter, Operation},
    fpga::{FPGADrive, GAIN_STM_BUF_SIZE_MAX, SAMPLING_FREQ_DIV_MAX, SAMPLING_FREQ_DIV_MIN},
    geometry::Device,
    operation::{
        stm::gain::reduced_phase::{PhaseFull, PhaseHalf},
        TypeTag,
    },
};

pub use gain_stm_mode::GainSTMMode;

use self::gain_stm_control_flags::GainSTMControlFlags;

pub struct GainSTMOp<G: Gain> {
    gains: Vec<G>,
    drives: Vec<HashMap<usize, Vec<Drive>>>,
    remains: HashMap<usize, usize>,
    sent: HashMap<usize, usize>,
    mode: GainSTMMode,
    freq_div: u32,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
}

impl<G: Gain> GainSTMOp<G> {
    pub fn new(
        gains: Vec<G>,
        mode: GainSTMMode,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
    ) -> Self {
        Self {
            gains,
            drives: Default::default(),
            remains: Default::default(),
            sent: Default::default(),
            mode,
            freq_div,
            start_idx,
            finish_idx,
        }
    }
}

impl<G: Gain> Operation for GainSTMOp<G> {
    fn init(
        &mut self,
        geometry: &crate::derive::prelude::Geometry,
    ) -> Result<(), crate::derive::prelude::AUTDInternalError> {
        if !(SAMPLING_FREQ_DIV_MIN..=SAMPLING_FREQ_DIV_MAX).contains(&self.freq_div) {
            return Err(AUTDInternalError::GainSTMFreqDivOutOfRange(self.freq_div));
        }

        self.drives = self
            .gains
            .iter()
            .map(|g| g.calc(geometry, GainFilter::All))
            .collect::<Result<_, _>>()?;

        match self.start_idx {
            Some(idx) if idx >= self.gains.len() as u16 => {
                return Err(AUTDInternalError::STMStartIndexOutOfRange)
            }
            _ => {}
        }
        match self.finish_idx {
            Some(idx) if idx >= self.gains.len() as u16 => {
                return Err(AUTDInternalError::STMFinishIndexOutOfRange)
            }
            _ => {}
        }

        if !(2..=GAIN_STM_BUF_SIZE_MAX).contains(&self.drives.len()) {
            return Err(AUTDInternalError::GainSTMSizeOutOfRange(self.drives.len()));
        }

        self.remains = geometry
            .devices()
            .map(|device| (device.idx(), self.drives.len()))
            .collect();

        self.sent = geometry.devices().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    fn required_size(&self, device: &Device) -> usize {
        if self.sent[&device.idx()] == 0 {
            std::mem::size_of::<TypeTag>()
                 + std::mem::size_of::<GainSTMControlFlags>()
                 + std::mem::size_of::<GainSTMMode>()
                 + std::mem::size_of::<u32>() // freq_div
                 + std::mem::size_of::<u16>() // start idx
                 + std::mem::size_of::<u16>() // finish idx
                 + device.num_transducers() * std::mem::size_of::<FPGADrive>()
        } else {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<FPGADrive>()
        }
    }

    fn pack(
        &mut self,
        device: &Device,
        tx: &mut [u8],
    ) -> Result<usize, crate::derive::prelude::AUTDInternalError> {
        assert!(self.remains[&device.idx()] > 0);

        tx[0] = TypeTag::GainSTM as u8;

        let sent = self.sent[&device.idx()];
        let mut offset =
            std::mem::size_of::<TypeTag>() + std::mem::size_of::<GainSTMControlFlags>();
        if sent == 0 {
            offset += std::mem::size_of::<GainSTMMode>()
         + std::mem::size_of::<u32>() // freq_div
         + std::mem::size_of::<u16>() // start idx
         + std::mem::size_of::<u16>(); // finish idx
        }
        assert!(tx.len() >= offset + device.num_transducers() * std::mem::size_of::<FPGADrive>());

        let mut f = GainSTMControlFlags::NONE;
        f.set(GainSTMControlFlags::STM_BEGIN, sent == 0);

        if sent == 0 {
            let mode = self.mode as u16;
            tx[2] = (mode & 0xFF) as u8;
            tx[3] = (mode >> 8) as u8;

            let freq_div = self.freq_div;
            tx[4] = (freq_div & 0xFF) as u8;
            tx[5] = ((freq_div >> 8) & 0xFF) as u8;
            tx[6] = ((freq_div >> 16) & 0xFF) as u8;
            tx[7] = ((freq_div >> 24) & 0xFF) as u8;

            f.set(GainSTMControlFlags::USE_START_IDX, self.start_idx.is_some());
            let start_idx = self.start_idx.unwrap_or(0);
            tx[8] = (start_idx & 0xFF) as u8;
            tx[9] = (start_idx >> 8) as u8;

            f.set(
                GainSTMControlFlags::USE_FINISH_IDX,
                self.finish_idx.is_some(),
            );
            let finish_idx = self.finish_idx.unwrap_or(0);
            tx[10] = (finish_idx & 0xFF) as u8;
            tx[11] = (finish_idx >> 8) as u8;
        }

        let mut send = 0;
        match self.mode {
            GainSTMMode::PhaseDutyFull => {
                let d = &self.drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        tx[offset..].as_mut_ptr() as *mut FPGADrive,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                send += 1;
            }
            GainSTMMode::PhaseFull => {
                let d = &self.drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        tx[offset..].as_mut_ptr() as *mut PhaseFull<0>,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                send += 1;
                if self.drives.len() > sent + 1 {
                    let d = &self.drives[sent + 1][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut PhaseFull<1>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
            }
            GainSTMMode::PhaseHalf => {
                let d = &self.drives[sent][&device.idx()];
                unsafe {
                    let dst = std::slice::from_raw_parts_mut(
                        tx[offset..].as_mut_ptr() as *mut PhaseHalf<0>,
                        d.len(),
                    );
                    dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                }
                send += 1;
                if self.drives.len() > sent + 1 {
                    let d = &self.drives[sent + 1][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut PhaseHalf<1>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
                if self.drives.len() > sent + 2 {
                    let d = &self.drives[sent + 2][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut PhaseHalf<2>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
                if self.drives.len() > sent + 3 {
                    let d = &self.drives[sent + 3][&device.idx()];
                    unsafe {
                        let dst = std::slice::from_raw_parts_mut(
                            tx[offset..].as_mut_ptr() as *mut PhaseHalf<3>,
                            d.len(),
                        );
                        dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
                    }
                    send += 1;
                }
            }
        }
        f.set(
            GainSTMControlFlags::STM_END,
            sent + send == self.drives.len(),
        );

        self.sent.insert(device.idx(), sent + send);

        tx[1] = f.bits() | ((send as u8 - 1) & 0x03) << 6;

        if sent == 0 {
            Ok(std::mem::size_of::<TypeTag>()
         + std::mem::size_of::<GainSTMControlFlags>()
         + std::mem::size_of::<GainSTMMode>()
         +  std::mem::size_of::<u32>() // freq_div
         + std::mem::size_of::<u16>() // start idx
         + std::mem::size_of::<u16>() // finish idx
         +device.num_transducers() * std::mem::size_of::<FPGADrive>())
        } else {
            Ok(std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<GainSTMControlFlags>()
                + device.num_transducers() * std::mem::size_of::<FPGADrive>())
        }
    }

    fn commit(&mut self, device: &Device) {
        self.remains
            .insert(device.idx(), self.drives.len() - self.sent[&device.idx()]);
    }

    fn remains(&self, device: &Device) -> usize {
        self.remains[&device.idx()]
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::{
        common::Amplitude,
        defined::PI,
        derive::prelude::Operation,
        fpga::{GAIN_STM_BUF_SIZE_MAX, SAMPLING_FREQ_DIV_MAX, SAMPLING_FREQ_DIV_MIN},
        geometry::tests::create_geometry,
        operation::{
            stm::gain::GainSTMOp,
            tests::{NullGain, TestGain},
        },
    };

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn gain_stm_phase_duty_full_op() {
        const GAIN_STM_SIZE: usize = 3;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: Amplitude::new_clamped(rng.gen_range(0.0..1.0)),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..SAMPLING_FREQ_DIV_MAX);
        let mut op = GainSTMOp::<_>::new(gains, GainSTMMode::PhaseDutyFull, freq_div, None, None);

        assert!(op.init(&geometry).is_ok());

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 1));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_ne!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 0);

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseDutyFull as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseDutyFull as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[0][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], FPGADrive::to_phase(g));
                    assert_eq!(d[1], FPGADrive::to_duty(g));
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_eq!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 0);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[1][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], FPGADrive::to_phase(g));
                    assert_eq!(d[1], FPGADrive::to_duty(g));
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_eq!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_ne!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 0);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[2][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], FPGADrive::to_phase(g));
                    assert_eq!(d[1], FPGADrive::to_duty(g));
                })
        });
    }

    #[test]
    fn gain_stm_phase_full_op() {
        const GAIN_STM_SIZE: usize = 5;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: Amplitude::new_clamped(rng.gen_range(0.0..1.0)),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..SAMPLING_FREQ_DIV_MAX);
        let mut op = GainSTMOp::<_>::new(gains, GainSTMMode::PhaseFull, freq_div, None, None);

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_ne!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 1);

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseFull as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseFull as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(gain_data[1][&dev.idx()].iter())
                .for_each(|((d, g0), g1)| {
                    assert_eq!(d[0], FPGADrive::to_phase(g0));
                    assert_eq!(d[1], FPGADrive::to_phase(g1));
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 4));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_eq!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 1);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[2][&dev.idx()].iter())
                .zip(gain_data[3][&dev.idx()].iter())
                .for_each(|((d, g0), g1)| {
                    assert_eq!(d[0], FPGADrive::to_phase(g0));
                    assert_eq!(d[1], FPGADrive::to_phase(g1));
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_eq!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_ne!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 0);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[4][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], FPGADrive::to_phase(g));
                })
        });
    }

    #[test]
    fn gain_stm_phase_half_op() {
        const GAIN_STM_SIZE: usize = 11;
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..GAIN_STM_SIZE)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: Amplitude::new_clamped(rng.gen_range(0.0..1.0)),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..GAIN_STM_SIZE)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..SAMPLING_FREQ_DIV_MAX);
        let mut op = GainSTMOp::<_>::new(gains, GainSTMMode::PhaseHalf, freq_div, None, None);

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE));

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 12 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(12 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 4));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_ne!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 3);

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                ((GainSTMMode::PhaseHalf as u16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((GainSTMMode::PhaseHalf as u16) >> 8) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 4], (freq_div & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);

            tx[FRAME_SIZE * dev.idx() + 12..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[0][&dev.idx()].iter())
                .zip(gain_data[1][&dev.idx()].iter())
                .zip(gain_data[2][&dev.idx()].iter())
                .zip(gain_data[3][&dev.idx()].iter())
                .for_each(|((((d, g0), g1), g2), g3)| {
                    assert_eq!(d[0] & 0x0F, FPGADrive::to_phase(g0) >> 4);
                    assert_eq!(d[0] >> 4, FPGADrive::to_phase(g1) >> 4);
                    assert_eq!(d[1] & 0x0F, FPGADrive::to_phase(g2) >> 4);
                    assert_eq!(d[1] >> 4, FPGADrive::to_phase(g3) >> 4);
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), GAIN_STM_SIZE - 8));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_eq!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 3);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[4][&dev.idx()].iter())
                .zip(gain_data[5][&dev.idx()].iter())
                .zip(gain_data[6][&dev.idx()].iter())
                .zip(gain_data[7][&dev.idx()].iter())
                .for_each(|((((d, g0), g1), g2), g3)| {
                    assert_eq!(d[0] & 0x0F, FPGADrive::to_phase(g0) >> 4);
                    assert_eq!(d[0] >> 4, FPGADrive::to_phase(g1) >> 4);
                    assert_eq!(d[1] & 0x0F, FPGADrive::to_phase(g2) >> 4);
                    assert_eq!(d[1] >> 4, FPGADrive::to_phase(g3) >> 4);
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2 + NUM_TRANS_IN_UNIT * 2));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(2 + NUM_TRANS_IN_UNIT * 2)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::GainSTM as u8);
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_eq!(flag & GainSTMControlFlags::STM_BEGIN.bits(), 0x00);
            assert_ne!(flag & GainSTMControlFlags::STM_END.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 1] >> 6, 2);

            tx[FRAME_SIZE * dev.idx() + 2..]
                .chunks(std::mem::size_of::<FPGADrive>())
                .zip(gain_data[8][&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0] & 0x0F, FPGADrive::to_phase(g) >> 4);
                })
        });
    }

    #[test]
    fn gain_stm_op_idx() {
        const FRAME_SIZE: usize = 12 + NUM_TRANS_IN_UNIT * 2;

        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let start_idx = rng.gen_range(0..2_u16);
        let finish_idx = rng.gen_range(0..2_u16);

        let mut rng = rand::thread_rng();

        let gain_data: Vec<HashMap<usize, Vec<Drive>>> = (0..2)
            .map(|_| {
                geometry
                    .devices()
                    .map(|dev| {
                        (
                            dev.idx(),
                            (0..dev.num_transducers())
                                .map(|_| Drive {
                                    amp: Amplitude::new_clamped(rng.gen_range(0.0..1.0)),
                                    phase: rng.gen_range(0.0..2.0 * PI),
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let gains: Vec<TestGain> = (0..2)
            .map(|i| TestGain {
                data: gain_data[i].clone(),
            })
            .collect();

        let mut op = GainSTMOp::<_>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            Some(finish_idx),
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_ne!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_ne!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], (finish_idx >> 8) as u8);
        });

        let mut op = GainSTMOp::<_>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            Some(start_idx),
            None,
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_ne!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_eq!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], 0x00);
        });

        let mut op = GainSTMOp::<_>::new(
            gains.clone(),
            GainSTMMode::PhaseDutyFull,
            SAMPLING_FREQ_DIV_MIN,
            None,
            Some(finish_idx),
        );
        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = tx[dev.idx() * FRAME_SIZE + 1];
            assert_eq!(flag & GainSTMControlFlags::USE_START_IDX.bits(), 0x00);
            assert_ne!(flag & GainSTMControlFlags::USE_FINISH_IDX.bits(), 0x00);

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 9], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 10], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 11], (finish_idx >> 8) as u8);
        });
    }

    #[test]
    fn gain_stm_op_buffer_out_of_range() {
        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let test = |n: usize| {
            let gains: Vec<NullGain> = (0..n).map(|_| NullGain {}).collect();

            let mut op = GainSTMOp::<_>::new(
                gains,
                GainSTMMode::PhaseDutyFull,
                SAMPLING_FREQ_DIV_MIN,
                None,
                None,
            );
            op.init(&geometry)
        };

        assert_eq!(test(1), Err(AUTDInternalError::GainSTMSizeOutOfRange(1)));
        assert_eq!(test(2), Ok(()));
        assert_eq!(test(GAIN_STM_BUF_SIZE_MAX), Ok(()));
        assert_eq!(
            test(GAIN_STM_BUF_SIZE_MAX + 1),
            Err(AUTDInternalError::GainSTMSizeOutOfRange(
                GAIN_STM_BUF_SIZE_MAX + 1
            ))
        );
    }

    #[test]
    fn gain_stm_op_freq_div_out_of_range() {
        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let test = |d: u32| {
            let gains: Vec<NullGain> = (0..2).map(|_| NullGain {}).collect();

            let mut op = GainSTMOp::<_>::new(gains, GainSTMMode::PhaseDutyFull, d, None, None);
            op.init(&geometry)
        };

        assert_eq!(
            test(SAMPLING_FREQ_DIV_MIN - 1),
            Err(AUTDInternalError::GainSTMFreqDivOutOfRange(
                SAMPLING_FREQ_DIV_MIN - 1
            ))
        );
        assert_eq!(test(SAMPLING_FREQ_DIV_MIN), Ok(()));
        assert_eq!(test(SAMPLING_FREQ_DIV_MAX), Ok(()));
    }

    #[test]
    fn gain_stm_op_stm_idx_out_of_range() {
        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let test = |n: usize, start_idx: Option<u16>, finish_idx: Option<u16>| {
            let mut op = GainSTMOp::new(
                (0..n).map(|_| NullGain {}).collect(),
                GainSTMMode::PhaseDutyFull,
                SAMPLING_FREQ_DIV_MIN,
                start_idx,
                finish_idx,
            );
            op.init(&geometry)
        };

        assert_eq!(test(10, Some(0), Some(0)), Ok(()));
        assert_eq!(test(10, Some(9), Some(0)), Ok(()));
        assert_eq!(
            test(10, Some(10), Some(0)),
            Err(AUTDInternalError::STMStartIndexOutOfRange)
        );

        assert_eq!(test(10, Some(0), Some(0)), Ok(()));
        assert_eq!(test(10, Some(0), Some(9)), Ok(()));
        assert_eq!(
            test(10, Some(0), Some(10)),
            Err(AUTDInternalError::STMFinishIndexOutOfRange)
        );
    }
}
