/*
 * File: advanced_phase.rs
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

use super::{GainControlFlags, GainOpDelegate};

use crate::{
    fpga::AdvancedDrivePhase,
    geometry::{Device, Transducer},
    operation::TypeTag,
};

pub struct GainOpAdvancedPhase {}

impl<T: Transducer> GainOpDelegate<T> for GainOpAdvancedPhase {
    fn pack(
        drives: &std::collections::HashMap<usize, Vec<crate::derive::prelude::Drive>>,
        remains: &std::collections::HashMap<usize, usize>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, crate::derive::prelude::AUTDInternalError> {
        assert_eq!(remains[&device.idx()], 1);

        tx[0] = TypeTag::Gain as u8;
        tx[1] = GainControlFlags::NONE.bits();

        let d = &drives[&device.idx()];
        assert!(tx.len() >= 2 + d.len() * std::mem::size_of::<AdvancedDrivePhase>());

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                tx[2..].as_mut_ptr() as *mut AdvancedDrivePhase,
                d.len(),
            );
            dst.iter_mut()
                .zip(d.iter())
                .zip(device.iter().map(|tr| tr.cycle()))
                .for_each(|((d, s), c)| d.set(s, c));
        }

        Ok(2 + d.len() * std::mem::size_of::<AdvancedDrivePhase>())
    }

    fn init(
        geometry: &crate::derive::prelude::Geometry<T>,
    ) -> Result<std::collections::HashMap<usize, usize>, crate::derive::prelude::AUTDInternalError>
    {
        Ok(geometry.devices().map(|device| (device.idx(), 1)).collect())
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::{
        defined::PI,
        derive::prelude::{AUTDInternalError, Drive, GainOp, Operation},
        geometry::{tests::create_geometry, AdvancedPhaseTransducer},
        operation::tests::{ErrGain, TestGain},
    };

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn gain_advanced_phase_op() {
        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

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
                            amp: rng.gen_range(0.0..1.0),
                            phase: rng.gen_range(0.0..2.0 * PI),
                        })
                        .collect(),
                )
            })
            .collect();
        let gain = TestGain { data };
        let mut op = GainOp::<AdvancedPhaseTransducer, TestGain>::new(gain.clone());

        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.required_size(dev),
                2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<AdvancedDrivePhase>()
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
            let flag = tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1];
            assert_eq!(flag & GainControlFlags::LEGACY.bits(), 0x00);
            assert_eq!(flag & GainControlFlags::DUTY.bits(), 0x00);
            tx.chunks(2)
                .skip((1 + NUM_TRANS_IN_UNIT) * dev.idx())
                .skip(1)
                .zip(gain.data[&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let phase = AdvancedDrivePhase::to_phase(g, tr.cycle());
                    assert_eq!(d[0], (phase & 0xFF) as u8);
                    assert_eq!(d[1], (phase >> 8) as u8);
                })
        });
    }

    #[test]
    fn error_gain_advanced_phase() {
        let geometry = create_geometry::<AdvancedPhaseTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let gain = ErrGain {};
        let mut op = GainOp::<AdvancedPhaseTransducer, ErrGain>::new(gain);

        assert_eq!(
            op.init(&geometry),
            Err(AUTDInternalError::GainError("test".to_owned()))
        );
    }
}
