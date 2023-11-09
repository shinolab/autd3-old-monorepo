/*
 * File: mod.rs
 * Project: gain
 * Created Date: 08/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    common::Drive, datagram::Gain, derive::prelude::GainFilter, fpga::FPGADrive, geometry::Device,
    operation::TypeTag,
};

use super::Operation;

pub struct GainOp<G: Gain> {
    gain: G,
    drives: HashMap<usize, Vec<Drive>>,
    remains: HashMap<usize, usize>,
}

impl<G: Gain> GainOp<G> {
    pub fn new(gain: G) -> Self {
        Self {
            gain,
            drives: Default::default(),
            remains: Default::default(),
        }
    }
}

impl<G: Gain> Operation for GainOp<G> {
    fn init(
        &mut self,
        geometry: &crate::derive::prelude::Geometry,
    ) -> Result<(), crate::derive::prelude::AUTDInternalError> {
        self.drives = self.gain.calc(geometry, GainFilter::All)?;
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn required_size(&self, device: &Device) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<u16>()
    }

    fn pack(
        &mut self,
        device: &Device,
        tx: &mut [u8],
    ) -> Result<usize, crate::derive::prelude::AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);

        let d = &self.drives[&device.idx()];
        assert!(tx.len() >= 2 + d.len() * std::mem::size_of::<FPGADrive>());

        tx[0] = TypeTag::Gain as u8;
        tx[1] = 0x01; // For v3 firmware compatibility

        unsafe {
            let dst =
                std::slice::from_raw_parts_mut(tx[2..].as_mut_ptr() as *mut FPGADrive, d.len());
            dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
        }

        Ok(2 + d.len() * std::mem::size_of::<FPGADrive>())
    }

    fn commit(&mut self, device: &Device) {
        self.remains
            .insert(device.idx(), self.remains[&device.idx()] - 1);
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
        derive::prelude::{AUTDInternalError, Drive, GainOp, Operation},
        geometry::tests::create_geometry,
        operation::tests::{ErrGain, TestGain},
    };

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn gain_op() {
        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx =
            vec![0x00u8; (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) * NUM_DEVICE];

        let mut rng = rand::thread_rng();
        let data = geometry
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
            .collect();
        let gain = TestGain { data };
        let mut op = GainOp::<TestGain>::new(gain.clone());

        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.required_size(dev),
                2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<FPGADrive>()
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
                TypeTag::Gain as u8
            );
            tx.chunks(2)
                .skip((1 + NUM_TRANS_IN_UNIT) * dev.idx())
                .skip(1)
                .zip(gain.data[&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], FPGADrive::to_phase(g));
                    assert_eq!(d[1], FPGADrive::to_duty(g));
                })
        });
    }

    #[test]
    fn error_gain() {
        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let gain = ErrGain {};
        let mut op = GainOp::<ErrGain>::new(gain);

        assert_eq!(
            op.init(&geometry),
            Err(AUTDInternalError::GainError("test".to_owned()))
        );
    }
}
