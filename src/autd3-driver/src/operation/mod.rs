/*
 * File: mod.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod clear;
mod filter;
mod gain;
mod info;
mod mod_delay;
mod modulation;
mod null;
mod silencer;
mod stm;
mod stop;
mod sync;
mod update_flag;

pub use clear::*;
pub use filter::*;
pub use gain::*;
pub use info::*;
pub use mod_delay::*;
pub use modulation::*;
pub use null::*;
pub use silencer::*;
pub use stm::*;
pub use stop::*;
pub use sync::*;
pub use update_flag::*;

use crate::{
    cpu::TxDatagram,
    error::AUTDInternalError,
    fpga::FPGAControlFlags,
    geometry::{Device, Geometry, Transducer},
};

#[repr(u8)]
pub enum TypeTag {
    NONE = 0x00,
    Clear = 0x01,
    Sync = 0x02,
    FirmwareInfo = 0x03,
    UpdateFlags = 0x04,
    Modulation = 0x10,
    ConfigureModDelay = 0x11,
    Silencer = 0x20,
    Gain = 0x30,
    FocusSTM = 0x40,
    GainSTM = 0x50,
    Filter = 0x60,
}

impl From<u8> for TypeTag {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NONE,
            0x01 => Self::Clear,
            0x02 => Self::Sync,
            0x03 => Self::FirmwareInfo,
            0x04 => Self::UpdateFlags,
            0x10 => Self::Modulation,
            0x11 => Self::ConfigureModDelay,
            0x20 => Self::Silencer,
            0x30 => Self::Gain,
            0x40 => Self::FocusSTM,
            0x50 => Self::GainSTM,
            0x60 => Self::Filter,
            _ => unimplemented!(),
        }
    }
}

pub trait Operation<T: Transducer> {
    fn init(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError>;
    fn required_size(&self, device: &Device<T>) -> usize;
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError>;
    fn commit(&mut self, device: &Device<T>);
    fn remains(&self, device: &Device<T>) -> usize;
}

impl<T: Transducer> Operation<T> for Box<dyn Operation<T>> {
    fn init(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        self.as_mut().init(geometry)
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        self.as_ref().required_size(device)
    }

    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        self.as_mut().pack(device, tx)
    }

    fn commit(&mut self, device: &Device<T>) {
        self.as_mut().commit(device)
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.as_ref().remains(device)
    }
}

pub struct OperationHandler {}

impl OperationHandler {
    pub fn is_finished<'a, T: Transducer + 'a, O1: Operation<T>, O2: Operation<T>>(
        op1: &mut O1,
        op2: &mut O2,
        geometry: &Geometry<T>,
    ) -> bool {
        geometry
            .devices()
            .all(|dev| op1.remains(dev) == 0 && op2.remains(dev) == 0)
    }

    pub fn init<'a, T: Transducer + 'a, O1: Operation<T>, O2: Operation<T>>(
        op1: &mut O1,
        op2: &mut O2,
        geometry: &Geometry<T>,
    ) -> Result<(), AUTDInternalError> {
        op1.init(geometry)?;
        op2.init(geometry)
    }

    pub fn pack<'a, T: Transducer + 'a, O1: Operation<T>, O2: Operation<T>>(
        op1: &mut O1,
        op2: &mut O2,
        geometry: &Geometry<T>,
        tx: &mut TxDatagram,
    ) -> Result<(), AUTDInternalError> {
        geometry
            .devices()
            .map(|dev| match (op1.remains(dev), op2.remains(dev)) {
                (0, 0) => Ok(()),
                (0, _) => Self::pack_dev(op2, dev, tx),
                (_, 0) => Self::pack_dev(op1, dev, tx),
                _ => {
                    let hedaer = tx.header_mut(dev.idx());
                    hedaer.msg_id = hedaer.msg_id.wrapping_add(1);
                    let mut f = FPGAControlFlags::NONE;
                    f.set(FPGAControlFlags::FORCE_FAN, dev.force_fan);
                    f.set(FPGAControlFlags::READS_FPGA_INFO, dev.reads_fpga_info);
                    hedaer.fpga_flag = f;
                    hedaer.slot_2_offset = 0;

                    let t = tx.payload_mut(dev.idx());
                    assert!(t.len() >= op1.required_size(dev));
                    let op1_size = op1.pack(dev, t)?;
                    op1.commit(dev);

                    if t.len() - op1_size >= op2.required_size(dev) {
                        tx.header_mut(dev.idx()).slot_2_offset = op1_size as u16;
                        let t = tx.payload_mut(dev.idx());
                        op2.pack(dev, &mut t[op1_size..])?;
                        op2.commit(dev);
                    }

                    Ok(())
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn pack_dev<T: Transducer, O: Operation<T>>(
        op: &mut O,
        dev: &Device<T>,
        tx: &mut TxDatagram,
    ) -> Result<(), AUTDInternalError> {
        if op.remains(dev) == 0 {
            return Ok(());
        }

        let hedaer = tx.header_mut(dev.idx());
        hedaer.msg_id = hedaer.msg_id.wrapping_add(1);
        let mut f = FPGAControlFlags::NONE;
        f.set(FPGAControlFlags::FORCE_FAN, dev.force_fan);
        f.set(FPGAControlFlags::READS_FPGA_INFO, dev.reads_fpga_info);
        hedaer.fpga_flag = f;
        hedaer.slot_2_offset = 0;

        op.pack(dev, tx.payload_mut(dev.idx()))?;
        op.commit(dev);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        cpu::{Header, EC_OUTPUT_FRAME_SIZE},
        geometry::{LegacyTransducer, UnitQuaternion, Vector3},
    };

    use super::*;

    struct OperationMock {
        pub initialized: HashMap<usize, bool>,
        pub pack_size: HashMap<usize, usize>,
        pub required_size: HashMap<usize, usize>,
        pub num_frames: HashMap<usize, usize>,
    }

    impl<T: Transducer> Operation<T> for OperationMock {
        fn init(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
            self.initialized = geometry
                .devices()
                .map(|dev| (dev.idx(), true))
                .collect::<HashMap<_, _>>();
            Ok(())
        }

        fn required_size(&self, device: &Device<T>) -> usize {
            self.required_size[&device.idx()]
        }

        fn pack(&mut self, device: &Device<T>, _: &mut [u8]) -> Result<usize, AUTDInternalError> {
            Ok(self.pack_size[&device.idx()])
        }

        fn commit(&mut self, device: &Device<T>) {
            *self.num_frames.get_mut(&device.idx()).unwrap() -= 1;
        }

        fn remains(&self, device: &Device<T>) -> usize {
            self.num_frames[&device.idx()]
        }
    }

    #[test]
    fn op_handler_test() {
        let geometry = Geometry::new(vec![Device::new(
            0,
            vec![LegacyTransducer::new(
                0,
                Vector3::zeros(),
                UnitQuaternion::identity(),
            )],
        )]);

        let mut op1 = OperationMock {
            initialized: Default::default(),
            pack_size: Default::default(),
            required_size: Default::default(),
            num_frames: Default::default(),
        };
        op1.pack_size.insert(0, 1);
        op1.required_size.insert(0, 2);
        op1.num_frames.insert(0, 3);

        let mut op2 = OperationMock {
            initialized: Default::default(),
            pack_size: Default::default(),
            required_size: Default::default(),
            num_frames: Default::default(),
        };
        op2.pack_size.insert(0, 1);
        op2.required_size.insert(0, 2);
        op2.num_frames.insert(0, 3);

        OperationHandler::init(&mut op1, &mut op2, &geometry).unwrap();

        assert!(op1.initialized[&0]);
        assert!(op2.initialized[&0]);
        assert!(!OperationHandler::is_finished(
            &mut op1, &mut op2, &geometry
        ));

        let mut tx = TxDatagram::new(1);

        OperationHandler::pack(&mut op1, &mut op2, &geometry, &mut tx).unwrap();
        assert_eq!(op1.num_frames[&0], 2);
        assert_eq!(op2.num_frames[&0], 2);
        assert!(!OperationHandler::is_finished(
            &mut op1, &mut op2, &geometry
        ));

        op1.pack_size.insert(
            0,
            EC_OUTPUT_FRAME_SIZE - std::mem::size_of::<Header>() - op2.required_size[&0],
        );

        OperationHandler::pack(&mut op1, &mut op2, &geometry, &mut tx).unwrap();
        assert_eq!(op1.num_frames[&0], 1);
        assert_eq!(op2.num_frames[&0], 1);
        assert!(!OperationHandler::is_finished(
            &mut op1, &mut op2, &geometry
        ));

        op1.pack_size.insert(
            0,
            EC_OUTPUT_FRAME_SIZE - std::mem::size_of::<Header>() - op2.required_size[&0] + 1,
        );
        OperationHandler::pack(&mut op1, &mut op2, &geometry, &mut tx).unwrap();
        assert_eq!(op1.num_frames[&0], 0);
        assert_eq!(op2.num_frames[&0], 1);
        assert!(!OperationHandler::is_finished(
            &mut op1, &mut op2, &geometry
        ));

        OperationHandler::pack(&mut op1, &mut op2, &geometry, &mut tx).unwrap();
        assert_eq!(op1.num_frames[&0], 0);
        assert_eq!(op2.num_frames[&0], 0);
        assert!(OperationHandler::is_finished(&mut op1, &mut op2, &geometry));
    }
}
