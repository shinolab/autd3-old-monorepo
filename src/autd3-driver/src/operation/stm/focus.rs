/*
 * File: focus.rs
 * Project: stm
 * Created Date: 29/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    defined::METER,
    error::AUTDInternalError,
    fpga::{STMFocus, FOCUS_STM_BUF_SIZE_MAX, FPGA_SUB_CLK_FREQ_DIV, SAMPLING_FREQ_DIV_MIN},
    geometry::{Device, Geometry, Transducer, Vector3},
    operation::{Operation, TypeTag},
};

use std::fmt;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct FocusSTMControlFlags : u8 {
        const NONE            = 0;
        const STM_BEGIN       = 1 << 0;
        const STM_END         = 1 << 1;
        const USE_START_IDX   = 1 << 2;
        const USE_FINISH_IDX  = 1 << 3;
    }
}

impl fmt::Display for FocusSTMControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(FocusSTMControlFlags::STM_BEGIN) {
            flags.push("STM_BEGIN")
        }
        if self.contains(FocusSTMControlFlags::STM_END) {
            flags.push("STM_END")
        }
        if self.contains(FocusSTMControlFlags::USE_START_IDX) {
            flags.push("USE_START_IDX")
        }
        if self.contains(FocusSTMControlFlags::USE_FINISH_IDX) {
            flags.push("USE_FINISH_IDX")
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

/// Control point for FocusSTM
#[derive(Clone, Debug, Copy)]
pub struct ControlPoint {
    /// Focal point
    point: Vector3,
    /// Duty shift
    /// Duty ratio of ultrasound will be `50% >> shift`.
    /// If `shift` is 0, duty ratio is 50%, which means the amplitude is the maximum.
    shift: u8,
}

impl ControlPoint {
    /// constructor (shift is 0)
    ///
    /// # Arguments
    ///
    /// * `point` - focal point
    ///
    pub fn new(point: Vector3) -> Self {
        Self { point, shift: 0 }
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `point` - focal point
    /// * `shift` - duty shift
    ///
    pub fn with_shift(self, shift: u8) -> Self {
        Self { shift, ..self }
    }

    pub fn point(&self) -> &Vector3 {
        &self.point
    }

    pub fn shift(&self) -> u8 {
        self.shift
    }
}

impl From<Vector3> for ControlPoint {
    fn from(point: Vector3) -> Self {
        Self::new(point)
    }
}

impl From<(Vector3, u8)> for ControlPoint {
    fn from((point, shift): (Vector3, u8)) -> Self {
        Self::new(point).with_shift(shift)
    }
}

impl From<&Vector3> for ControlPoint {
    fn from(point: &Vector3) -> Self {
        Self::new(*point)
    }
}

impl From<&(Vector3, u8)> for ControlPoint {
    fn from((point, shift): &(Vector3, u8)) -> Self {
        Self::new(*point).with_shift(*shift)
    }
}

pub struct FocusSTMOp {
    remains: HashMap<usize, usize>,
    sent: HashMap<usize, usize>,
    points: Vec<ControlPoint>,
    freq_div: u32,
    start_idx: Option<u16>,
    finish_idx: Option<u16>,
}

impl FocusSTMOp {
    pub fn new(
        points: Vec<ControlPoint>,
        freq_div: u32,
        start_idx: Option<u16>,
        finish_idx: Option<u16>,
    ) -> Self {
        Self {
            points,
            remains: Default::default(),
            sent: Default::default(),
            freq_div,
            start_idx,
            finish_idx,
        }
    }
}

impl<T: Transducer> Operation<T> for FocusSTMOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert!(self.remains[&device.idx()] > 0);

        tx[0] = TypeTag::FocusSTM as u8;

        let sent = self.sent[&device.idx()];
        let mut offset = std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<FocusSTMControlFlags>()
            + std::mem::size_of::<u16>(); // size
        if sent == 0 {
            offset += std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u32>() // sound_speed
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>(); // finish idx
        }
        let send_bytes =
            ((self.points.len() - sent) * std::mem::size_of::<STMFocus>()).min(tx.len() - offset);
        let send_num = send_bytes / std::mem::size_of::<STMFocus>();
        assert!(send_num > 0);

        let mut f = FocusSTMControlFlags::NONE;
        f.set(FocusSTMControlFlags::STM_BEGIN, sent == 0);
        f.set(
            FocusSTMControlFlags::STM_END,
            sent + send_num == self.points.len(),
        );

        tx[2] = (send_num & 0xFF) as u8;
        tx[3] = (send_num >> 8) as u8;

        if sent == 0 {
            let freq_div = self.freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;
            tx[4] = (freq_div & 0xFF) as u8;
            tx[5] = ((freq_div >> 8) & 0xFF) as u8;
            tx[6] = ((freq_div >> 16) & 0xFF) as u8;
            tx[7] = ((freq_div >> 24) & 0xFF) as u8;

            let sound_speed = (device.sound_speed / METER * 1024.0).round() as u32;
            tx[8] = (sound_speed & 0xFF) as u8;
            tx[9] = ((sound_speed >> 8) & 0xFF) as u8;
            tx[10] = ((sound_speed >> 16) & 0xFF) as u8;
            tx[11] = ((sound_speed >> 24) & 0xFF) as u8;

            let start_idx = self.start_idx.unwrap_or(0);
            tx[12] = (start_idx & 0xFF) as u8;
            tx[13] = (start_idx >> 8) as u8;
            f.set(
                FocusSTMControlFlags::USE_START_IDX,
                self.start_idx.is_some(),
            );

            let finish_idx = self.finish_idx.unwrap_or(0);
            tx[14] = (finish_idx & 0xFF) as u8;
            tx[15] = (finish_idx >> 8) as u8;
            f.set(
                FocusSTMControlFlags::USE_FINISH_IDX,
                self.finish_idx.is_some(),
            );
        }
        tx[1] = f.bits();

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                tx[offset..].as_mut_ptr() as *mut STMFocus,
                send_num,
            );
            dst.iter_mut()
                .zip(self.points.iter().skip(sent).take(send_num))
                .for_each(|(d, p)| {
                    let lp = device.to_local(p.point());
                    d.set(lp.x, lp.y, lp.z, p.shift());
                })
        }

        self.sent.insert(device.idx(), sent + send_num);
        if sent == 0 {
            Ok(std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<FocusSTMControlFlags>()
            + std::mem::size_of::<u16>() // size
            + std::mem::size_of::<u32>() // freq_div
            + std::mem::size_of::<u32>() // sound_speed
            + std::mem::size_of::<u16>() // start idx
            + std::mem::size_of::<u16>() // finish idx
            + std::mem::size_of::<STMFocus>() * send_num)
        } else {
            Ok(std::mem::size_of::<TypeTag>()
            + std::mem::size_of::<FocusSTMControlFlags>()
            + std::mem::size_of::<u16>() // size
            + std::mem::size_of::<STMFocus>() * send_num)
        }
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        if self.sent[&device.idx()] == 0 {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<FocusSTMControlFlags>()
                + std::mem::size_of::<u16>() // size
                + std::mem::size_of::<u32>() // freq_div
                + std::mem::size_of::<u32>() // sound_speed
                + std::mem::size_of::<u16>() // start idx
                + std::mem::size_of::<u16>() // finish idx
                + std::mem::size_of::<STMFocus>()
        } else {
            std::mem::size_of::<TypeTag>()
                + std::mem::size_of::<FocusSTMControlFlags>()
                + std::mem::size_of::<u16>() // size
                + std::mem::size_of::<STMFocus>()
        }
    }

    fn init(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        if self.points.len() < 2 || self.points.len() > FOCUS_STM_BUF_SIZE_MAX {
            return Err(AUTDInternalError::FocusSTMPointSizeOutOfRange(
                self.points.len(),
            ));
        }
        if self.freq_div < SAMPLING_FREQ_DIV_MIN
            || self.freq_div > u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32
        {
            return Err(AUTDInternalError::FocusSTMFreqDivOutOfRange(self.freq_div));
        }

        self.remains = geometry
            .devices()
            .map(|device| (device.idx(), self.points.len()))
            .collect();
        self.sent = geometry.devices().map(|device| (device.idx(), 0)).collect();

        Ok(())
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains
            .insert(device.idx(), self.points.len() - self.sent[&device.idx()]);
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::{
        defined::MILLIMETER,
        fpga::SAMPLING_FREQ_DIV_MIN,
        geometry::{tests::create_geometry, LegacyTransducer},
    };

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn focus_stm_op() {
        const FOCUS_STM_SIZE: usize = 100;
        const FRAME_SIZE: usize = 16 + 8 * FOCUS_STM_SIZE;

        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let points: Vec<ControlPoint> = (0..FOCUS_STM_SIZE)
            .map(|_| {
                ControlPoint::new(Vector3::new(
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(0.0 * MILLIMETER..500.0 * MILLIMETER),
                ))
                .with_shift(rng.gen_range(0..0xFF))
            })
            .collect();
        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);

        let mut op = FocusSTMOp::new(points.clone(), freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 16 + 8));

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), FOCUS_STM_SIZE));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::FocusSTM as u8);
            let flag = FocusSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(FocusSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(FocusSTMControlFlags::STM_END));
            assert!(!flag.contains(FocusSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(FocusSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                (FOCUS_STM_SIZE & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((FOCUS_STM_SIZE >> 8) & 0xFF) as u8
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

            let sound_speed = (dev.sound_speed / METER * 1024.0).round() as u32;
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (sound_speed & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 9],
                ((sound_speed >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 10],
                ((sound_speed >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 11],
                ((sound_speed >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 12], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 13], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 14], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 15], 0x00);

            tx[FRAME_SIZE * dev.idx() + 16..]
                .chunks(std::mem::size_of::<STMFocus>())
                .zip(points.iter())
                .for_each(|(d, p)| {
                    let mut f = STMFocus { buf: [0x0000; 4] };
                    f.set(p.point.x, p.point.y, p.point.z, p.shift);
                    assert_eq!(d[0], (f.buf[0] & 0xFF) as u8);
                    assert_eq!(d[1], ((f.buf[0] >> 8) & 0xFF) as u8);
                    assert_eq!(d[2], (f.buf[1] & 0xFF) as u8);
                    assert_eq!(d[3], ((f.buf[1] >> 8) & 0xFF) as u8);
                    assert_eq!(d[4], (f.buf[2] & 0xFF) as u8);
                    assert_eq!(d[5], ((f.buf[2] >> 8) & 0xFF) as u8);
                    assert_eq!(d[6], (f.buf[3] & 0xFF) as u8);
                    assert_eq!(d[7], ((f.buf[3] >> 8) & 0xFF) as u8);
                })
        });
    }

    #[test]
    fn focus_stm_op_div() {
        const FRAME_SIZE: usize = 30;
        const FOCUS_STM_SIZE: usize = (FRAME_SIZE - 16) / std::mem::size_of::<STMFocus>()
            + (FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>() * 2;

        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let points: Vec<ControlPoint> = (0..FOCUS_STM_SIZE)
            .map(|_| {
                ControlPoint::new(Vector3::new(
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(0.0 * MILLIMETER..500.0 * MILLIMETER),
                ))
                .with_shift(rng.gen_range(0..0xFF))
            })
            .collect();
        let freq_div: u32 =
            rng.gen_range(SAMPLING_FREQ_DIV_MIN..u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32);
        let mut op = FocusSTMOp::new(points.clone(), freq_div, None, None);
        let freq_div = freq_div * FPGA_SUB_CLK_FREQ_DIV as u32;

        assert!(op.init(&geometry).is_ok());

        // First frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 16 + 8));

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), FOCUS_STM_SIZE));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(
                    dev,
                    &mut tx[dev.idx() * FRAME_SIZE..(dev.idx() + 1) * FRAME_SIZE]
                ),
                Ok(16
                    + (FRAME_SIZE - 16) / std::mem::size_of::<STMFocus>()
                        * std::mem::size_of::<STMFocus>())
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.remains(dev),
                (FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>() * 2
            )
        });

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::FocusSTM as u8);
            let flag = FocusSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(FocusSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(FocusSTMControlFlags::STM_END));
            assert!(!flag.contains(FocusSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(FocusSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                (((FRAME_SIZE - 16) / std::mem::size_of::<STMFocus>()) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((((FRAME_SIZE - 16) / std::mem::size_of::<STMFocus>()) >> 8) & 0xFF) as u8
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

            let sound_speed = (dev.sound_speed / METER * 1024.0).round() as u32;
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 8], (sound_speed & 0xFF) as u8);
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 9],
                ((sound_speed >> 8) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 10],
                ((sound_speed >> 16) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 11],
                ((sound_speed >> 24) & 0xFF) as u8
            );

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 12], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 13], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 14], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 15], 0x00);

            tx[FRAME_SIZE * dev.idx() + 16..FRAME_SIZE * (dev.idx() + 1)]
                .chunks(std::mem::size_of::<STMFocus>())
                .zip(
                    points
                        .iter()
                        .take((FRAME_SIZE - 16) / std::mem::size_of::<STMFocus>()),
                )
                .for_each(|(d, p)| {
                    let mut f = STMFocus { buf: [0x0000; 4] };
                    f.set(p.point.x, p.point.y, p.point.z, p.shift);
                    assert_eq!(d[0], (f.buf[0] & 0xFF) as u8);
                    assert_eq!(d[1], ((f.buf[0] >> 8) & 0xFF) as u8);
                    assert_eq!(d[2], (f.buf[1] & 0xFF) as u8);
                    assert_eq!(d[3], ((f.buf[1] >> 8) & 0xFF) as u8);
                    assert_eq!(d[4], (f.buf[2] & 0xFF) as u8);
                    assert_eq!(d[5], ((f.buf[2] >> 8) & 0xFF) as u8);
                    assert_eq!(d[6], (f.buf[3] & 0xFF) as u8);
                    assert_eq!(d[7], ((f.buf[3] >> 8) & 0xFF) as u8);
                })
        });

        // Second frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 4 + std::mem::size_of::<STMFocus>()));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(
                    dev,
                    &mut tx[dev.idx() * FRAME_SIZE..(dev.idx() + 1) * FRAME_SIZE]
                ),
                Ok(4 + (FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()
                    * std::mem::size_of::<STMFocus>())
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.remains(dev),
                (FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()
            )
        });

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::FocusSTM as u8);
            let flag = FocusSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(FocusSTMControlFlags::STM_BEGIN));
            assert!(!flag.contains(FocusSTMControlFlags::STM_END));
            assert!(!flag.contains(FocusSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(FocusSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                (((FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((((FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()) >> 8) & 0xFF) as u8
            );

            tx[FRAME_SIZE * dev.idx() + 4..FRAME_SIZE * (dev.idx() + 1)]
                .chunks(std::mem::size_of::<STMFocus>())
                .zip(
                    points
                        .iter()
                        .skip((FRAME_SIZE - 16) / std::mem::size_of::<STMFocus>())
                        .take((FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()),
                )
                .for_each(|(d, p)| {
                    let mut f = STMFocus { buf: [0x0000; 4] };
                    f.set(p.point.x, p.point.y, p.point.z, p.shift);
                    assert_eq!(d[0], (f.buf[0] & 0xFF) as u8);
                    assert_eq!(d[1], ((f.buf[0] >> 8) & 0xFF) as u8);
                    assert_eq!(d[2], (f.buf[1] & 0xFF) as u8);
                    assert_eq!(d[3], ((f.buf[1] >> 8) & 0xFF) as u8);
                    assert_eq!(d[4], (f.buf[2] & 0xFF) as u8);
                    assert_eq!(d[5], ((f.buf[2] >> 8) & 0xFF) as u8);
                    assert_eq!(d[6], (f.buf[3] & 0xFF) as u8);
                    assert_eq!(d[7], ((f.buf[3] >> 8) & 0xFF) as u8);
                })
        });

        // Final frame
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 4 + std::mem::size_of::<STMFocus>()));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(
                    dev,
                    &mut tx[dev.idx() * FRAME_SIZE..(dev.idx() + 1) * FRAME_SIZE]
                ),
                Ok(4 + (FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()
                    * std::mem::size_of::<STMFocus>())
            );
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * FRAME_SIZE], TypeTag::FocusSTM as u8);
            let flag = FocusSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(FocusSTMControlFlags::STM_BEGIN));
            assert!(flag.contains(FocusSTMControlFlags::STM_END));
            assert!(!flag.contains(FocusSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(FocusSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 2],
                (((FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()) & 0xFF) as u8
            );
            assert_eq!(
                tx[dev.idx() * FRAME_SIZE + 3],
                ((((FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()) >> 8) & 0xFF) as u8
            );

            tx[FRAME_SIZE * dev.idx() + 4..FRAME_SIZE * (dev.idx() + 1)]
                .chunks(std::mem::size_of::<STMFocus>())
                .zip(
                    points
                        .iter()
                        .skip(
                            (FRAME_SIZE - 16) / std::mem::size_of::<STMFocus>()
                                + (FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>(),
                        )
                        .take((FRAME_SIZE - 4) / std::mem::size_of::<STMFocus>()),
                )
                .for_each(|(d, p)| {
                    let mut f = STMFocus { buf: [0x0000; 4] };
                    f.set(p.point.x, p.point.y, p.point.z, p.shift);
                    assert_eq!(d[0], (f.buf[0] & 0xFF) as u8);
                    assert_eq!(d[1], ((f.buf[0] >> 8) & 0xFF) as u8);
                    assert_eq!(d[2], (f.buf[1] & 0xFF) as u8);
                    assert_eq!(d[3], ((f.buf[1] >> 8) & 0xFF) as u8);
                    assert_eq!(d[4], (f.buf[2] & 0xFF) as u8);
                    assert_eq!(d[5], ((f.buf[2] >> 8) & 0xFF) as u8);
                    assert_eq!(d[6], (f.buf[3] & 0xFF) as u8);
                    assert_eq!(d[7], ((f.buf[3] >> 8) & 0xFF) as u8);
                })
        });
    }

    #[test]
    fn focus_stm_op_idx() {
        const FOCUS_STM_SIZE: usize = 100;
        const FRAME_SIZE: usize = 16 + 8 * FOCUS_STM_SIZE;

        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = vec![0x00u8; FRAME_SIZE * NUM_DEVICE];

        let mut rng = rand::thread_rng();

        let start_idx = rng.gen_range(0..FOCUS_STM_SIZE as u16);
        let finish_idx = rng.gen_range(0..FOCUS_STM_SIZE as u16);

        let points: Vec<ControlPoint> = (0..FOCUS_STM_SIZE)
            .map(|_| ControlPoint::new(Vector3::zeros()))
            .collect();

        let mut op = FocusSTMOp::new(
            points.clone(),
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
            let flag = FocusSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(FocusSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(FocusSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 12], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 13], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 14], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 15], (finish_idx >> 8) as u8);
        });

        let mut op = FocusSTMOp::new(points.clone(), SAMPLING_FREQ_DIV_MIN, Some(start_idx), None);

        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.pack(dev, &mut tx[dev.idx() * FRAME_SIZE..]),
                Ok(FRAME_SIZE)
            );
            op.commit(dev);
        });

        geometry.devices().for_each(|dev| {
            let flag = FocusSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(flag.contains(FocusSTMControlFlags::USE_START_IDX));
            assert!(!flag.contains(FocusSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 12], (start_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 13], (start_idx >> 8) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 14], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 15], 0x00);
        });

        let mut op = FocusSTMOp::new(
            points.clone(),
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
            let flag = FocusSTMControlFlags::from_bits_truncate(tx[dev.idx() * FRAME_SIZE + 1]);
            assert!(!flag.contains(FocusSTMControlFlags::USE_START_IDX));
            assert!(flag.contains(FocusSTMControlFlags::USE_FINISH_IDX));

            assert_eq!(tx[dev.idx() * FRAME_SIZE + 12], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 13], 0x00);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 14], (finish_idx & 0xFF) as u8);
            assert_eq!(tx[dev.idx() * FRAME_SIZE + 15], (finish_idx >> 8) as u8);
        });
    }

    #[test]
    fn focus_stm_op_buffer_out_of_range() {
        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut rng = rand::thread_rng();

        let points: Vec<ControlPoint> = (0..FOCUS_STM_BUF_SIZE_MAX)
            .map(|_| {
                ControlPoint::new(Vector3::new(
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(0.0 * MILLIMETER..500.0 * MILLIMETER),
                ))
                .with_shift(rng.gen_range(0..0xFF))
            })
            .collect();
        let mut op = FocusSTMOp::new(points, SAMPLING_FREQ_DIV_MIN, None, None);
        assert!(op.init(&geometry).is_ok());

        let points: Vec<ControlPoint> = (0..FOCUS_STM_BUF_SIZE_MAX + 1)
            .map(|_| {
                ControlPoint::new(Vector3::new(
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(0.0 * MILLIMETER..500.0 * MILLIMETER),
                ))
                .with_shift(rng.gen_range(0..0xFF))
            })
            .collect();

        let mut op = FocusSTMOp::new(points, SAMPLING_FREQ_DIV_MIN, None, None);
        assert!(op.init(&geometry).is_err());

        let points: Vec<ControlPoint> = (0..1)
            .map(|_| {
                ControlPoint::new(Vector3::new(
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(0.0 * MILLIMETER..500.0 * MILLIMETER),
                ))
                .with_shift(rng.gen_range(0..0xFF))
            })
            .collect();

        let mut op = FocusSTMOp::new(points, SAMPLING_FREQ_DIV_MIN, None, None);
        assert!(op.init(&geometry).is_err());
    }

    #[test]
    fn focus_stm_op_freq_div_out_of_range() {
        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut rng = rand::thread_rng();

        let points: Vec<ControlPoint> = (0..100)
            .map(|_| {
                ControlPoint::new(Vector3::new(
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(-500.0 * MILLIMETER..500.0 * MILLIMETER),
                    rng.gen_range(0.0 * MILLIMETER..500.0 * MILLIMETER),
                ))
                .with_shift(rng.gen_range(0..0xFF))
            })
            .collect();

        let mut op = FocusSTMOp::new(points.clone(), SAMPLING_FREQ_DIV_MIN, None, None);
        assert!(op.init(&geometry).is_ok());

        let mut op = FocusSTMOp::new(points.clone(), SAMPLING_FREQ_DIV_MIN - 1, None, None);
        assert!(op.init(&geometry).is_err());

        let mut op = FocusSTMOp::new(
            points.clone(),
            u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32,
            None,
            None,
        );
        assert!(op.init(&geometry).is_ok());

        let mut op = FocusSTMOp::new(
            points.clone(),
            u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32 + 1,
            None,
            None,
        );
        assert!(op.init(&geometry).is_err());
    }
}
