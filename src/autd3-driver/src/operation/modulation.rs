/*
 * File: modulation.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, fmt};

use crate::{
    defined::{float, PI},
    error::AUTDInternalError,
    fpga::{MOD_BUF_SIZE_MAX, SAMPLING_FREQ_DIV_MIN},
    geometry::{Device, Transducer},
    operation::{Operation, TypeTag},
};

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct ModulationControlFlags : u8 {
        const NONE      = 0;
        const MOD_BEGIN = 1 << 0;
        const MOD_END   = 1 << 1;
    }
}

impl fmt::Display for ModulationControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(ModulationControlFlags::MOD_BEGIN) {
            flags.push("MOD_BEGIN")
        }
        if self.contains(ModulationControlFlags::MOD_END) {
            flags.push("MOD_END")
        }
        if self.is_empty() {
            flags.push("NONE")
        }
        write!(
            f,
            "{}",
            flags
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

#[derive(Default)]
pub struct ModulationOp {
    buf: Vec<u8>,
    freq_div: u32,
    remains: HashMap<usize, usize>,
    sent: HashMap<usize, usize>,
}

impl ModulationOp {
    pub fn to_duty(amp: &float) -> u8 {
        (amp.clamp(0., 1.).asin() * 2.0 / PI * 255.0) as u8
    }

    pub fn new(buf: Vec<float>, freq_div: u32) -> Self {
        Self {
            buf: buf.iter().map(Self::to_duty).collect(),
            freq_div,
            remains: HashMap::new(),
            sent: HashMap::new(),
        }
    }
}

impl<T: Transducer> Operation<T> for ModulationOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert!(self.remains[&device.idx()] > 0);

        tx[0] = TypeTag::Modulation as u8;

        let sent = self.sent[&device.idx()];
        let mut offset = 4;
        if sent == 0 {
            offset += 4;
        }
        let mod_size = (self.buf.len() - sent).min(tx.len() - offset);
        assert!(mod_size > 0);

        let mut f = ModulationControlFlags::NONE;
        f.set(ModulationControlFlags::MOD_BEGIN, sent == 0);
        f.set(
            ModulationControlFlags::MOD_END,
            sent + mod_size == self.buf.len(),
        );
        tx[1] = f.bits();

        tx[2] = (mod_size & 0xFF) as u8;
        tx[3] = (mod_size >> 8) as u8;

        if sent == 0 {
            tx[4] = (self.freq_div & 0xFF) as u8;
            tx[5] = ((self.freq_div >> 8) & 0xFF) as u8;
            tx[6] = ((self.freq_div >> 16) & 0xFF) as u8;
            tx[7] = ((self.freq_div >> 24) & 0xFF) as u8;
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                self.buf[sent..].as_ptr(),
                (&mut tx[offset..]).as_mut_ptr() as *mut u8,
                mod_size,
            )
        }

        self.sent.insert(device.idx(), sent + mod_size);
        if sent == 0 {
            Ok(2 + std::mem::size_of::<u16>() + std::mem::size_of::<u32>() + mod_size)
        } else {
            Ok(2 + std::mem::size_of::<u16>() + mod_size)
        }
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        if self.sent[&device.idx()] == 0 {
            7
        } else {
            5
        }
    }

    fn init(&mut self, devices: &[&Device<T>]) -> Result<(), AUTDInternalError> {
        if self.buf.len() < 2 || self.buf.len() > MOD_BUF_SIZE_MAX {
            return Err(AUTDInternalError::ModulationSizeOutOfRange(self.buf.len()));
        }
        if self.freq_div < SAMPLING_FREQ_DIV_MIN {
            return Err(AUTDInternalError::ModFreqDivOutOfRange(self.freq_div));
        }

        self.remains = devices
            .iter()
            .map(|device| (device.idx(), self.buf.len()))
            .collect();
        self.sent = devices.iter().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains
            .insert(device.idx(), self.buf.len() - self.sent[&device.idx()]);
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::{
        fpga::SAMPLING_FREQ_DIV_MIN,
        geometry::{tests::create_device, LegacyTransducer},
    };

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn modulation_op() {
        const MOD_SIZE: usize = 100;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; (2 + 2 + 4 + MOD_SIZE) * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let buf: Vec<float> = (0..MOD_SIZE).map(|_| rng.gen()).collect();
        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = ModulationOp::new(buf.clone(), freq_div);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 7));

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), MOD_SIZE));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE)..]),
                Ok(2 + 2 + 4 + MOD_SIZE)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(
                tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE)],
                TypeTag::Modulation as u8
            );
            let flag = ModulationControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE) + 1],
            );
            assert!(flag.contains(ModulationControlFlags::MOD_BEGIN));
            assert!(flag.contains(ModulationControlFlags::MOD_END));

            assert_eq!(
                tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE) + 2],
                (MOD_SIZE & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE) + 3],
                ((MOD_SIZE >> 8) & 0xFF) as u8
            );

            assert_eq!(
                tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE) + 4],
                (freq_div & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE) + 5],
                ((freq_div >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE) + 6],
                ((freq_div >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * (2 + 2 + 4 + MOD_SIZE) + 7],
                ((freq_div >> 24) & 0xFF) as u8
            );

            tx.iter()
                .skip((2 + 2 + 4 + MOD_SIZE) * dev.idx())
                .skip(8)
                .zip(buf.iter())
                .for_each(|(&d, m)| {
                    assert_eq!(d, ModulationOp::to_duty(m));
                })
        });
    }

    #[test]
    fn modulation_op_div() {
        const FRAME_SIZE: usize = 30;
        const MOD_SIZE: usize = FRAME_SIZE - 4 + FRAME_SIZE * 2;

        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; (2 + 2 + FRAME_SIZE) * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let buf: Vec<float> = (0..MOD_SIZE).map(|_| rng.gen()).collect();

        let mut op = ModulationOp::new(buf.clone(), SAMPLING_FREQ_DIV_MIN);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        // First frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 7));

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), MOD_SIZE));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(
                    dev,
                    &mut tx
                        [dev.idx() * (2 + 2 + FRAME_SIZE)..(dev.idx() + 1) * (2 + 2 + FRAME_SIZE)]
                ),
                Ok(2 + 2 + FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), MOD_SIZE - (FRAME_SIZE - 4)));

        devices.iter().for_each(|dev| {
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE)],
                TypeTag::Modulation as u8
            );
            let flag = ModulationControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 1],
            );
            assert!(flag.contains(ModulationControlFlags::MOD_BEGIN));
            assert!(!flag.contains(ModulationControlFlags::MOD_END));

            let mod_size = FRAME_SIZE - 4;
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 2],
                (mod_size & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 3],
                ((mod_size >> 8) & 0xFF) as u8
            );

            tx.iter()
                .skip((2 + 2 + FRAME_SIZE) * dev.idx())
                .skip(8)
                .zip(buf.iter().take(mod_size))
                .for_each(|(&d, m)| {
                    assert_eq!(d, ModulationOp::to_duty(m));
                })
        });

        // Second frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 5));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(
                    dev,
                    &mut tx
                        [dev.idx() * (2 + 2 + FRAME_SIZE)..(dev.idx() + 1) * (2 + 2 + FRAME_SIZE)]
                ),
                Ok(2 + 2 + FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), MOD_SIZE - (FRAME_SIZE - 4) - FRAME_SIZE));

        devices.iter().for_each(|dev| {
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE)],
                TypeTag::Modulation as u8
            );
            let flag = ModulationControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 1],
            );
            assert!(!flag.contains(ModulationControlFlags::MOD_BEGIN));
            assert!(!flag.contains(ModulationControlFlags::MOD_END));

            let mod_size = FRAME_SIZE;
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 2],
                (mod_size & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 3],
                ((mod_size >> 8) & 0xFF) as u8
            );

            tx.iter()
                .skip((2 + 2 + FRAME_SIZE) * dev.idx())
                .skip(4)
                .zip(buf.iter().skip(FRAME_SIZE - 4).take(mod_size))
                .for_each(|(&d, m)| {
                    assert_eq!(d, ModulationOp::to_duty(m));
                })
        });

        // Final frame
        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 5));

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.pack(
                    dev,
                    &mut tx
                        [dev.idx() * (2 + 2 + FRAME_SIZE)..(dev.idx() + 1) * (2 + 2 + FRAME_SIZE)]
                ),
                Ok(2 + 2 + FRAME_SIZE)
            );
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE)],
                TypeTag::Modulation as u8
            );
            let flag = ModulationControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 1],
            );
            assert!(!flag.contains(ModulationControlFlags::MOD_BEGIN));
            assert!(flag.contains(ModulationControlFlags::MOD_END));

            let mod_size = FRAME_SIZE;
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 2],
                (mod_size & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * (2 + 2 + FRAME_SIZE) + 3],
                ((mod_size >> 8) & 0xFF) as u8
            );

            tx.iter()
                .skip((2 + 2 + FRAME_SIZE) * dev.idx())
                .skip(4)
                .zip(buf.iter().skip(FRAME_SIZE - 4 + FRAME_SIZE).take(mod_size))
                .for_each(|(&d, m)| {
                    assert_eq!(d, ModulationOp::to_duty(m));
                })
        });
    }

    #[test]
    fn modulation_op_buffer_out_of_range() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let buf: Vec<float> = (0..MOD_BUF_SIZE_MAX).map(|_| rng.gen()).collect();
        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = ModulationOp::new(buf.clone(), freq_div);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut rng = rand::thread_rng();

        let buf: Vec<float> = (0..MOD_BUF_SIZE_MAX + 1).map(|_| rng.gen()).collect();
        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = ModulationOp::new(buf.clone(), freq_div);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());

        let mut rng = rand::thread_rng();

        let buf: Vec<float> = (0..1).map(|_| rng.gen()).collect();
        let freq_div: u32 = rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX);

        let mut op = ModulationOp::new(buf.clone(), freq_div);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn modulation_op_freq_div_out_of_range() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let buf: Vec<float> = (0..MOD_BUF_SIZE_MAX).map(|_| rng.gen()).collect();

        let mut op = ModulationOp::new(buf.clone(), SAMPLING_FREQ_DIV_MIN);
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut op = ModulationOp::new(buf.clone(), u32::MAX);
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        let mut op = ModulationOp::new(buf.clone(), SAMPLING_FREQ_DIV_MIN - 1);
        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_err());
    }
}
