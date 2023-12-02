/*
 * File: mod_delay.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    derive::prelude::Transducer,
    error::AUTDInternalError,
    geometry::{Device, Geometry},
    operation::{Operation, TypeTag},
};

pub struct ConfigureModDelayOp<F: Fn(&Device, &Transducer) -> u16> {
    remains: HashMap<usize, usize>,
    f: F,
}

impl<F: Fn(&Device, &Transducer) -> u16> ConfigureModDelayOp<F> {
    pub fn new(f: F) -> Self {
        Self {
            remains: Default::default(),
            f,
        }
    }
}

impl<F: Fn(&Device, &Transducer) -> u16> Operation for ConfigureModDelayOp<F> {
    fn pack(&mut self, device: &Device, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);

        assert!(tx.len() >= 2 + device.num_transducers() * std::mem::size_of::<u16>());

        tx[0] = TypeTag::ConfigureModDelay as u8;

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                tx[2..].as_mut_ptr() as *mut u16,
                device.num_transducers(),
            );
            dst.iter_mut()
                .zip(device.iter())
                .for_each(|(d, s)| *d = (self.f)(device, s));
        }

        Ok(2 + device.num_transducers() * std::mem::size_of::<u16>())
    }

    fn required_size(&self, device: &Device) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<u16>()
    }

    fn init(&mut self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device) {
        self.remains.insert(device.idx(), 0);
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::geometry::tests::create_geometry;

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn mod_delay_op() {
        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx =
            vec![0x00u8; (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) * NUM_DEVICE];

        let map = geometry
            .devices()
            .flat_map(|dev| {
                let mut rng = rand::thread_rng();
                dev.iter()
                    .map(move |tr| ((dev.idx(), tr.idx()), rng.gen_range(0x0000..0xFFFFu16)))
            })
            .collect::<HashMap<_, _>>();

        let map2 = map.clone();
        let mut op = ConfigureModDelayOp::new(move |dev, tr| map2[&(dev.idx(), tr.idx())]);

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
                TypeTag::ConfigureModDelay as u8
            );
            tx.chunks(2)
                .skip((1 + NUM_TRANS_IN_UNIT) * dev.idx())
                .skip(1)
                .zip(dev.iter())
                .for_each(|(d, tr)| {
                    let delay = map[&(dev.idx(), tr.idx())];
                    assert_eq!(d[0], (delay & 0xFF) as u8);
                    assert_eq!(d[1], (delay >> 8) as u8);
                })
        });
    }
}
