/*
 * File: gain.rs
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

use std::{collections::HashMap, fmt};

use crate::{
    datagram::{Gain, GainFilter},
    defined::{float, Drive},
    error::AUTDInternalError,
    fpga::{AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive},
    geometry::{
        AdvancedPhaseTransducer, AdvancedTransducer, Device, Geometry, LegacyTransducer, Transducer,
    },
    operation::{Operation, TypeTag},
};

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct GainControlFlags : u8 {
        const NONE    = 0;
        const LEGACY  = 1 << 0;
        const DUTY    = 1 << 1;
    }
}

impl fmt::Display for GainControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(GainControlFlags::LEGACY) {
            flags.push("LEGACY")
        }
        if self.contains(GainControlFlags::DUTY) {
            flags.push("DUTY")
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

pub struct GainOp<T: Transducer, G: Gain<T>> {
    gain: G,
    drives: HashMap<usize, Vec<Drive>>,
    remains: HashMap<usize, usize>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Transducer, G: Gain<T>> GainOp<T, G> {
    pub fn new(gain: G) -> Self {
        Self {
            gain,
            drives: Default::default(),
            remains: Default::default(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn pack_legacy(
        drives: &HashMap<usize, Vec<Drive>>,
        remains: &HashMap<usize, usize>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        assert_eq!(remains[&device.idx()], 1);

        let d = &drives[&device.idx()];
        assert!(tx.len() >= 2 + d.len() * std::mem::size_of::<LegacyDrive>());

        tx[0] = TypeTag::Gain as u8;
        tx[1] = GainControlFlags::LEGACY.bits();

        unsafe {
            let dst =
                std::slice::from_raw_parts_mut(tx[2..].as_mut_ptr() as *mut LegacyDrive, d.len());
            dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
        }

        Ok(2 + d.len() * std::mem::size_of::<LegacyDrive>())
    }

    pub fn pack_advanced(
        drives: &HashMap<usize, Vec<Drive>>,
        remains: &HashMap<usize, usize>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        tx[0] = TypeTag::Gain as u8;

        if remains[&device.idx()] == 2 {
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
        } else if remains[&device.idx()] == 1 {
            tx[1] = GainControlFlags::DUTY.bits();

            let d = &drives[&device.idx()];
            assert!(tx.len() >= 2 + d.len() * std::mem::size_of::<AdvancedDriveDuty>());

            unsafe {
                let dst = std::slice::from_raw_parts_mut(
                    tx[2..].as_mut_ptr() as *mut AdvancedDriveDuty,
                    d.len(),
                );
                dst.iter_mut()
                    .zip(d.iter())
                    .zip(device.iter().map(|tr| tr.cycle()))
                    .for_each(|((d, s), c)| d.set(s, c));
            }

            Ok(2 + d.len() * std::mem::size_of::<AdvancedDriveDuty>())
        } else {
            unreachable!()
        }
    }

    pub fn pack_advanced_phase(
        drives: &HashMap<usize, Vec<Drive>>,
        remains: &HashMap<usize, usize>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
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

    pub fn required_size_impl(device: &Device<T>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<u16>()
    }
}

impl<G: Gain<LegacyTransducer>> Operation<LegacyTransducer> for GainOp<LegacyTransducer, G> {
    fn pack(
        &mut self,
        device: &Device<LegacyTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        Self::pack_legacy(&self.drives, &self.remains, device, tx)
    }

    fn required_size(&self, device: &Device<LegacyTransducer>) -> usize {
        Self::required_size_impl(device)
    }

    fn init(&mut self, geometry: &Geometry<LegacyTransducer>) -> Result<(), AUTDInternalError> {
        self.drives = self.gain.calc(geometry, GainFilter::All)?;
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<LegacyTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<LegacyTransducer>) {
        self.remains
            .insert(device.idx(), self.remains[&device.idx()] - 1);
    }
}

impl<G: Gain<AdvancedTransducer>> Operation<AdvancedTransducer> for GainOp<AdvancedTransducer, G> {
    fn pack(
        &mut self,
        device: &Device<AdvancedTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        Self::pack_advanced(&self.drives, &self.remains, device, tx)
    }

    fn required_size(&self, device: &Device<AdvancedTransducer>) -> usize {
        Self::required_size_impl(device)
    }

    fn init(&mut self, geometry: &Geometry<AdvancedTransducer>) -> Result<(), AUTDInternalError> {
        self.drives = self.gain.calc(geometry, GainFilter::All)?;
        self.remains = geometry.devices().map(|device| (device.idx(), 2)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<AdvancedTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<AdvancedTransducer>) {
        self.remains
            .insert(device.idx(), self.remains[&device.idx()] - 1);
    }
}

impl<G: Gain<AdvancedPhaseTransducer>> Operation<AdvancedPhaseTransducer>
    for GainOp<AdvancedPhaseTransducer, G>
{
    fn pack(
        &mut self,
        device: &Device<AdvancedPhaseTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        Self::pack_advanced_phase(&self.drives, &self.remains, device, tx)
    }

    fn required_size(&self, device: &Device<AdvancedPhaseTransducer>) -> usize {
        Self::required_size_impl(device)
    }

    fn init(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<(), AUTDInternalError> {
        self.drives = self.gain.calc(geometry, GainFilter::All)?;
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<AdvancedPhaseTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<AdvancedPhaseTransducer>) {
        self.remains
            .insert(device.idx(), self.remains[&device.idx()] - 1);
    }
}

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
    use crate::{
        datagram::GainAsAny,
        defined::PI,
        geometry::{tests::create_geometry, LegacyTransducer},
    };

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[derive(Clone)]
    pub struct TestGain {
        pub data: HashMap<usize, Vec<Drive>>,
    }

    impl GainAsAny for TestGain {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl<T: Transducer> Gain<T> for TestGain {
        fn calc(
            &self,
            _geometry: &Geometry<T>,
            _filter: GainFilter,
        ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
            Ok(self.data.clone())
        }
    }

    #[test]
    fn gain_legacy_op() {
        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

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
        let mut op = GainOp::<LegacyTransducer, TestGain>::new(gain.clone());

        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.required_size(dev),
                2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<LegacyDrive>()
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
            let flag = GainControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1],
            );
            assert!(flag.contains(GainControlFlags::LEGACY));
            assert!(!flag.contains(GainControlFlags::DUTY));
            tx.chunks(2)
                .skip((1 + NUM_TRANS_IN_UNIT) * dev.idx())
                .skip(1)
                .zip(gain.data[&dev.idx()].iter())
                .for_each(|(d, g)| {
                    assert_eq!(d[0], LegacyDrive::to_phase(g));
                    assert_eq!(d[1], LegacyDrive::to_duty(g));
                })
        });
    }

    #[test]
    fn gain_advanced_op() {
        let geometry = create_geometry::<AdvancedTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

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
        let mut op = GainOp::<AdvancedTransducer, TestGain>::new(gain.clone());

        assert!(op.init(&geometry).is_ok());

        geometry.devices().for_each(|dev| {
            assert_eq!(
                op.required_size(dev),
                2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<AdvancedDrivePhase>()
            )
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 2));

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
            .for_each(|dev| assert_eq!(op.remains(dev), 1));

        geometry.devices().for_each(|dev| {
            assert_eq!(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>())],
                TypeTag::Gain as u8
            );
            let flag = GainControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1],
            );
            assert!(!flag.contains(GainControlFlags::LEGACY));
            assert!(!flag.contains(GainControlFlags::DUTY));
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
            let flag = GainControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1],
            );
            assert!(!flag.contains(GainControlFlags::LEGACY));
            assert!(flag.contains(GainControlFlags::DUTY));
            tx.chunks(2)
                .skip((1 + NUM_TRANS_IN_UNIT) * dev.idx())
                .skip(1)
                .zip(gain.data[&dev.idx()].iter())
                .zip(dev.iter())
                .for_each(|((d, g), tr)| {
                    let duty = AdvancedDriveDuty::to_duty(g, tr.cycle());
                    assert_eq!(d[0], (duty & 0xFF) as u8);
                    assert_eq!(d[1], (duty >> 8) as u8);
                })
        });
    }

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
            let flag = GainControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1],
            );
            assert!(!flag.contains(GainControlFlags::LEGACY));
            assert!(!flag.contains(GainControlFlags::DUTY));
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
            let flag = GainControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1],
            );
            assert!(!flag.contains(GainControlFlags::LEGACY));
            assert!(flag.contains(GainControlFlags::DUTY));
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
