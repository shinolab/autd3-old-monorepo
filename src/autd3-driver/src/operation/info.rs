/*
 * File: info.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    error::AUTDInternalError,
    geometry::{Device, Geometry, Transducer},
    operation::{Operation, TypeTag},
};

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum FirmwareInfoType {
    CPUVersionMajor = 0x01,
    CPUVersionMinor = 0x02,
    FPGAVersionMajor = 0x03,
    FPGAVersionMinor = 0x04,
    FPGAFunctions = 0x05,
    Clear = 0x06,
}

#[derive(Default)]
pub struct FirmInfoOp {
    remains: HashMap<usize, usize>,
}

impl<T: Transducer> Operation<T> for FirmInfoOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        tx[0] = TypeTag::FirmwareInfo as u8;
        match self.remains[&device.idx()] {
            6 => {
                tx[1] = FirmwareInfoType::CPUVersionMajor as u8;
            }
            5 => {
                tx[1] = FirmwareInfoType::CPUVersionMinor as u8;
            }
            4 => {
                tx[1] = FirmwareInfoType::FPGAVersionMajor as u8;
            }
            3 => {
                tx[1] = FirmwareInfoType::FPGAVersionMinor as u8;
            }
            2 => {
                tx[1] = FirmwareInfoType::FPGAFunctions as u8;
            }
            1 => {
                tx[1] = FirmwareInfoType::Clear as u8;
            }
            _ => unreachable!(),
        }

        Ok(2)
    }

    fn required_size(&self, _: &Device<T>) -> usize {
        2
    }

    fn init(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        self.remains = geometry.devices().map(|device| (device.idx(), 6)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains.insert(device.idx(), self.remains(device) - 1);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::{tests::create_geometry, LegacyTransducer};

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn info_op() {
        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = [0x00u8; 2 * NUM_DEVICE];

        let mut op = FirmInfoOp::default();

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 2));

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 6));

        geometry.devices().for_each(|dev| {
            assert!(op.pack(dev, &mut tx[dev.idx() * 2..]).is_ok());
            op.commit(dev);
        });
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 5));
        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * 2], TypeTag::FirmwareInfo as u8);
            let flag = tx[dev.idx() * 2 + 1];
            assert_eq!(flag, FirmwareInfoType::CPUVersionMajor as u8);
        });

        geometry.devices().for_each(|dev| {
            assert!(op.pack(dev, &mut tx[dev.idx() * 2..]).is_ok());
            op.commit(dev);
        });
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 4));
        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * 2], TypeTag::FirmwareInfo as u8);
            let flag = tx[dev.idx() * 2 + 1];
            assert_eq!(flag, FirmwareInfoType::CPUVersionMinor as u8);
        });

        geometry.devices().for_each(|dev| {
            assert!(op.pack(dev, &mut tx[dev.idx() * 2..]).is_ok());
            op.commit(dev);
        });
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 3));
        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * 2], TypeTag::FirmwareInfo as u8);
            let flag = tx[dev.idx() * 2 + 1];
            assert_eq!(flag, FirmwareInfoType::FPGAVersionMajor as u8);
        });

        geometry.devices().for_each(|dev| {
            assert!(op.pack(dev, &mut tx[dev.idx() * 2..]).is_ok());
            op.commit(dev);
        });
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 2));
        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * 2], TypeTag::FirmwareInfo as u8);
            let flag = tx[dev.idx() * 2 + 1];
            assert_eq!(flag, FirmwareInfoType::FPGAVersionMinor as u8);
        });

        geometry.devices().for_each(|dev| {
            assert!(op.pack(dev, &mut tx[dev.idx() * 2..]).is_ok());
            op.commit(dev);
        });
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 1));
        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * 2], TypeTag::FirmwareInfo as u8);
            let flag = tx[dev.idx() * 2 + 1];
            assert_eq!(flag, FirmwareInfoType::FPGAFunctions as u8);
        });

        geometry.devices().for_each(|dev| {
            assert!(op.pack(dev, &mut tx[dev.idx() * 2..]).is_ok());
            op.commit(dev);
        });
        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));
        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * 2], TypeTag::FirmwareInfo as u8);
            let flag = tx[dev.idx() * 2 + 1];
            assert_eq!(flag, FirmwareInfoType::Clear as u8);
        });
    }

    #[test]
    #[should_panic]
    fn info_op_panic() {
        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);
        let mut tx = [0x00u8; 2 * NUM_DEVICE];

        let mut op = FirmInfoOp::default();

        assert!(op.init(&geometry).is_ok());

        for _ in 0..7 {
            geometry.devices().for_each(|dev| {
                assert!(op.pack(dev, &mut tx[dev.idx() * 2..]).is_ok());
                op.commit(dev);
            });
        }
    }
}
