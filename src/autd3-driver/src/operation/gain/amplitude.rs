/*
 * File: amplitude.rs
 * Project: gain
 * Created Date: 08/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub use super::GainControlFlags;

use std::collections::HashMap;

use crate::{
    defined::{float, Drive},
    error::AUTDInternalError,
    fpga::AdvancedDriveDuty,
    geometry::{Device, Geometry, Transducer},
    operation::{Operation, TypeTag},
};

pub struct AmplitudeOp {
    amp: float,
    remains: HashMap<usize, usize>,
}

impl AmplitudeOp {
    pub fn new(amp: float) -> Self {
        Self {
            amp,
            remains: Default::default(),
        }
    }
}

impl<T: Transducer> Operation<T> for AmplitudeOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        tx[0] = TypeTag::Gain as u8;

        tx[1] = (GainControlFlags::NONE | GainControlFlags::DUTY).bits();

        assert!(
            tx.len() >= 2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>()
        );

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                tx[2..].as_mut_ptr() as *mut AdvancedDriveDuty,
                device.num_transducers(),
            );
            dst.iter_mut()
                .zip(device.iter().map(|tr| tr.cycle()))
                .for_each(|(d, c)| {
                    d.set(
                        &Drive {
                            amp: self.amp,
                            phase: 0.,
                        },
                        c,
                    )
                });
        }

        Ok(2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>())
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>()
    }

    fn init(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains
            .insert(device.idx(), self.remains[&device.idx()] - 1);
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::geometry::{tests::create_geometry, AdvancedPhaseTransducer};

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn amplitude_op() {
        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx =
            vec![0x00u8; (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) * NUM_DEVICE];

        let mut rng = rand::thread_rng();
        let amp: float = rng.gen_range(0.0..1.0);
        let mut op = AmplitudeOp::new(amp);

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 1));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.required_size(dev),
                2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<AdvancedDriveDuty>()
            )
        });

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
                TypeTag::Gain as u8
            );
            let flag = tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1];
            assert_eq!(flag & GainControlFlags::LEGACY.bits(), 0x00);
            assert_ne!(flag & GainControlFlags::DUTY.bits(), 0x00);
            tx.chunks(2)
                .skip((1 + NUM_TRANS_IN_UNIT) * dev.idx())
                .skip(1)
                .zip(dev.iter())
                .for_each(|(d, tr)| {
                    let duty = AdvancedDriveDuty::to_duty(&Drive { amp, phase: 0. }, tr.cycle());
                    assert_eq!(d[0], (duty & 0xFF) as u8);
                    assert_eq!(d[1], (duty >> 8) as u8);
                })
        });
    }
}
