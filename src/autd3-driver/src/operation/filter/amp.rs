/*
 * File: amp.rs
 * Project: filter
 * Created Date: 06/10/2023
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
    fpga::FilterDuty,
    geometry::{Device, Geometry, Transducer},
    operation::{Operation, TypeTag},
};

use super::FilterType;

#[derive(Default)]
pub struct ConfigureAmpFilterOp {
    remains: HashMap<usize, usize>,
}

impl<T: Transducer> Operation<T> for ConfigureAmpFilterOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);

        assert!(tx.len() >= 2 + device.num_transducers() * std::mem::size_of::<FilterDuty>());

        tx[0] = TypeTag::Filter as u8;
        tx[1] = FilterType::AddDuty as u8;

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                tx[2..].as_mut_ptr() as *mut FilterDuty,
                device.num_transducers(),
            );
            dst.iter_mut()
                .zip(device.iter())
                .for_each(|(d, tr)| d.set(tr.amp_filter(), tr.cycle()));
        }

        Ok(2 + device.num_transducers() * std::mem::size_of::<FilterDuty>())
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<FilterDuty>()
    }

    fn init(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains.insert(device.idx(), 0);
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::geometry::{tests::create_geometry, LegacyTransducer};

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn filter_amp_op() {
        let mut geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx =
            vec![0x00u8; (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) * NUM_DEVICE];

        let mut rng = rand::thread_rng();
        geometry.devices_mut().for_each(|dev| {
            dev.iter_mut().for_each(|tr| {
                tr.set_amp_filter(rng.gen_range(-1.0..1.0));
            })
        });

        let mut op = ConfigureAmpFilterOp::default();

        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.required_size(dev),
                2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()
            )
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 1));

        geometry.devices().for_each(|dev| {
            assert!(op
                .pack(
                    dev,
                    &mut tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>())..]
                )
                .is_ok());
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>())],
                TypeTag::Filter as u8
            );
            assert_eq!(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1],
                FilterType::AddDuty as u8
            );
            tx.chunks(2)
                .skip((1 + NUM_TRANS_IN_UNIT) * dev.idx())
                .skip(1)
                .zip(dev.iter())
                .for_each(|(d, tr)| {
                    let duty = FilterDuty::to_duty(tr.amp_filter(), tr.cycle());
                    assert_eq!(d[0], (duty & 0xFF) as u8);
                    assert_eq!(d[1], (duty >> 8) as u8);
                })
        });
    }
}
