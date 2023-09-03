/*
 * File: gain.rs
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
    defined::Drive,
    error::AUTDInternalError,
    fpga::{AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive},
    gain::{Gain, GainFilter},
    geometry::{AdvancedTransducer, Device, LegacyTransducer, Transducer},
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
}

impl<G: Gain<LegacyTransducer>> Operation<LegacyTransducer> for GainOp<LegacyTransducer, G> {
    fn pack(
        &mut self,
        device: &Device<LegacyTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);

        let d = &self.drives[&device.idx()];
        assert!(tx.len() >= 2 + d.len() * std::mem::size_of::<LegacyDrive>());

        tx[0] = TypeTag::Gain as u8;
        tx[1] = GainControlFlags::LEGACY.bits();

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                (&mut tx[2..]).as_mut_ptr() as *mut LegacyDrive,
                d.len(),
            );
            dst.iter_mut().zip(d.iter()).for_each(|(d, s)| d.set(s));
        }

        Ok(2 + d.len() * std::mem::size_of::<LegacyDrive>())
    }

    fn required_size(&self, device: &Device<LegacyTransducer>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<LegacyDrive>()
    }

    fn init(&mut self, devices: &[&Device<LegacyTransducer>]) -> Result<(), AUTDInternalError> {
        self.drives = self.gain.calc(devices, GainFilter::All)?;
        self.remains = devices.iter().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<LegacyTransducer>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<LegacyTransducer>) {
        self.remains.insert(device.idx(), 0);
    }
}

impl<G: Gain<AdvancedTransducer>> Operation<AdvancedTransducer> for GainOp<AdvancedTransducer, G> {
    fn pack(
        &mut self,
        device: &Device<AdvancedTransducer>,
        tx: &mut [u8],
    ) -> Result<usize, AUTDInternalError> {
        tx[0] = TypeTag::Gain as u8;

        if self.remains[&device.idx()] == 2 {
            tx[1] = GainControlFlags::NONE.bits();

            let d = &self.drives[&device.idx()];
            assert!(tx.len() > 2 + d.len() * std::mem::size_of::<AdvancedDrivePhase>());

            unsafe {
                let dst = std::slice::from_raw_parts_mut(
                    (&mut tx[2..]).as_mut_ptr() as *mut AdvancedDrivePhase,
                    d.len(),
                );
                dst.iter_mut()
                    .zip(d.iter())
                    .zip(device.iter().map(|tr| tr.cycle()))
                    .for_each(|((d, s), c)| d.set(s, c));
            }

            Ok(2 + d.len() * std::mem::size_of::<AdvancedDrivePhase>())
        } else if self.remains[&device.idx()] == 1 {
            tx[1] = GainControlFlags::DUTY.bits();

            let d = &self.drives[&device.idx()];
            assert!(tx.len() > 2 + d.len() * std::mem::size_of::<AdvancedDriveDuty>());

            unsafe {
                let dst = std::slice::from_raw_parts_mut(
                    (&mut tx[2..]).as_mut_ptr() as *mut AdvancedDriveDuty,
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

    fn required_size(&self, device: &Device<AdvancedTransducer>) -> usize {
        if self.remains(device) == 2 {
            2 + device.num_transducers() * std::mem::size_of::<AdvancedDrivePhase>()
        } else if self.remains(device) == 1 {
            2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>()
        } else {
            unreachable!()
        }
    }

    fn init(&mut self, devices: &[&Device<AdvancedTransducer>]) -> Result<(), AUTDInternalError> {
        self.drives = self.gain.calc(devices, GainFilter::All)?;
        self.remains = devices.iter().map(|device| (device.idx(), 2)).collect();
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

// impl GainOp for GainAdvanced {
//     fn new<F: Fn() -> Vec<Vec<u16>>>(drives: Vec<Vec<Drive>>, cycles_fn: F) -> Self {
//         Self {
//             remains: 2,
//             drives,
//             cycles: cycles_fn(),
//         }
//     }
// }

// impl GainAdvanced {
//     fn pack_duty(&mut self, tx: &[&mut [u8]]) -> Result<(), DriverError> {
//         assert_eq!(self.remains, 1);
//         assert_eq!(tx.len(), self.drives.len());

//         for ((t, d), c) in tx
//             .iter_mut()
//             .zip(self.drives.iter())
//             .zip(self.cycles.iter())
//         {
//             t[0] = TypeTag::GainAdvancedDuty as u8;

//             unsafe {
//                 let dst = std::slice::from_raw_parts_mut(
//                     (&mut t[1..]).as_mut_ptr() as *mut AdvancedDriveDuty,
//                     d.len(),
//                 );
//                 dst.iter_mut()
//                     .zip(d.iter())
//                     .zip(c.iter())
//                     .for_each(|((d, s), &c)| d.set(s, c));
//             }
//         }

//         self.remains -= 1;
//         Ok(())
//     }

//     fn pack_phase(&mut self, tx: &[&mut [u8]]) -> Result<(), DriverError> {
//         assert_eq!(self.remains, 2);
//         assert_eq!(tx.len(), self.drives.len());

//         for ((t, d), c) in tx
//             .iter_mut()
//             .zip(self.drives.iter())
//             .zip(self.cycles.iter())
//         {
//             t[0] = TypeTag::GainAdvancedPhase as u8;

//             unsafe {
//                 let dst = std::slice::from_raw_parts_mut(
//                     (&mut t[1..]).as_mut_ptr() as *mut AdvancedDrivePhase,
//                     d.len(),
//                 );
//                 dst.iter_mut()
//                     .zip(d.iter())
//                     .zip(c.iter())
//                     .for_each(|((d, s), &c)| d.set(s, c));
//             }
//         }

//         self.remains -= 1;
//         Ok(())
//     }
// }

// impl Operation for GainAdvanced {
//     fn pack(&mut self, tx: &[&mut [u8]]) -> Result<(), DriverError> {
//         assert!(self.remains == 1 || self.remains == 2);

//         if self.remains == 2 {
//             self.pack_phase(tx)?;
//         } else {
//             self.pack_duty(tx)?;
//         }

//         Ok(())
//     }

//     fn required_size(&self) -> usize {
//         1 + self.drives.iter().map(|d| d.len()).max().unwrap() * 2
//     }

//     fn init(&mut self) {
//         self.remains = 2;
//     }

//     fn remains(&self) -> usize {
//         self.remains
//     }
// }

// pub struct GainAdvancedPhase {
//     remains: usize,
//     drives: Vec<Vec<Drive>>,
//     cycles: Vec<Vec<u16>>,
// }

// impl GainOp for GainAdvancedPhase {
//     fn new<F: Fn() -> Vec<Vec<u16>>>(drives: Vec<Vec<Drive>>, cycles_fn: F) -> Self {
//         Self {
//             remains: 1,
//             drives,
//             cycles: cycles_fn(),
//         }
//     }
// }

// impl Operation for GainAdvancedPhase {
//     fn pack(&mut self, tx: &[&mut [u8]]) -> Result<(), DriverError> {
//         assert_eq!(self.remains, 1);
//         assert_eq!(tx.len(), self.drives.len());

//         for ((t, d), c) in tx
//             .iter_mut()
//             .zip(self.drives.iter())
//             .zip(self.cycles.iter())
//         {
//             t[0] = TypeTag::GainAdvancedPhase as u8;

//             unsafe {
//                 let dst = std::slice::from_raw_parts_mut(
//                     (&mut t[1..]).as_mut_ptr() as *mut AdvancedDrivePhase,
//                     d.len(),
//                 );
//                 dst.iter_mut()
//                     .zip(d.iter())
//                     .zip(c.iter())
//                     .for_each(|((d, s), &c)| d.set(s, c));
//             }
//         }

//         self.remains -= 1;
//         Ok(())
//     }

//     fn required_size(&self) -> usize {
//         1 + self.drives.iter().map(|d| d.len()).max().unwrap() * 2
//     }

//     fn init(&mut self) {
//         self.remains = 1;
//     }

//     fn remains(&self) -> usize {
//         self.remains
//     }
// }

// pub struct GainAdvancedDuty {
//     remains: usize,
//     drives: Vec<Vec<Drive>>,
//     cycles: Vec<Vec<u16>>,
// }

// impl GainOp for GainAdvancedDuty {
//     fn new<F: Fn() -> Vec<Vec<u16>>>(drives: Vec<Vec<Drive>>, cycles_fn: F) -> Self {
//         Self {
//             remains: 1,
//             drives,
//             cycles: cycles_fn(),
//         }
//     }
// }

// impl Operation for GainAdvancedDuty {
//     fn pack(&mut self, tx: &[&mut [u8]]) -> Result<(), DriverError> {
//         assert_eq!(self.remains, 1);
//         assert_eq!(tx.len(), self.drives.len());

//         for ((t, d), c) in tx
//             .iter_mut()
//             .zip(self.drives.iter())
//             .zip(self.cycles.iter())
//         {
//             t[0] = TypeTag::GainAdvancedDuty as u8;

//             unsafe {
//                 let dst = std::slice::from_raw_parts_mut(
//                     (&mut t[1..]).as_mut_ptr() as *mut AdvancedDriveDuty,
//                     d.len(),
//                 );
//                 dst.iter_mut()
//                     .zip(d.iter())
//                     .zip(c.iter())
//                     .for_each(|((d, s), &c)| d.set(s, c));
//             }
//         }

//         self.remains -= 1;
//         Ok(())
//     }

//     fn required_size(&self) -> usize {
//         1 + self.drives.iter().map(|d| d.len()).max().unwrap() * 2
//     }

//     fn init(&mut self) {
//         self.remains = 1;
//     }

//     fn remains(&self) -> usize {
//         self.remains
//     }
// }

// #[cfg(test)]
// mod test {
//     use rand::prelude::*;

//     use crate::{AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive};

//     use super::*;

//     const NUM_TRANS_IN_UNIT: usize = 249;

//     #[test]
//     fn legacy_gain() {
//         let mut tx = TxDatagram::new(&[
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//         ]);

//         let mut rng = rand::thread_rng();

//         let drives = (0..NUM_TRANS_IN_UNIT * 10)
//             .map(|_| Drive {
//                 amp: rng.gen_range(0.0..1.0),
//                 phase: rng.gen_range(0.0..1.0),
//             })
//             .collect::<Vec<_>>();

//         let mut op = GainLegacy::new(drives.clone(), || vec![4096; NUM_TRANS_IN_UNIT * 10]);
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());

//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         tx.body_raw_mut()
//             .iter()
//             .zip(drives.iter())
//             .for_each(|(&d, drive)| {
//                 assert_eq!((d & 0xFF) as u8, LegacyDrive::to_phase(drive));
//                 assert_eq!((d >> 8) as u8, LegacyDrive::to_duty(drive));
//             });
//         assert_eq!(tx.num_bodies, 10);

//         op.pack(&mut tx).unwrap();
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         assert_eq!(tx.num_bodies, 0);

//         op.init();
//         assert!(!op.is_finished());
//     }

//     #[test]
//     fn advanced_gain() {
//         let mut tx = TxDatagram::new(&[
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//         ]);

//         let mut rng = rand::thread_rng();

//         let drives = (0..NUM_TRANS_IN_UNIT * 10)
//             .map(|_| Drive {
//                 amp: rng.gen_range(0.0..1.0),
//                 phase: rng.gen_range(0.0..1.0),
//             })
//             .collect::<Vec<_>>();
//         let cycles = (0..NUM_TRANS_IN_UNIT * 10)
//             .map(|_| rng.gen_range(2u16..0xFFFFu16))
//             .collect::<Vec<_>>();
//         let mut op = GainAdvanced::new(drives.clone(), || cycles.clone());
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(!op.is_finished());
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
//         assert!(!tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         (0..NUM_TRANS_IN_UNIT * 10).for_each(|i| {
//             assert_eq!(
//                 tx.body_raw_mut()[i],
//                 AdvancedDrivePhase::to_phase(&drives[i], cycles[i])
//             );
//         });
//         assert_eq!(tx.num_bodies, 10);

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
//         assert!(!tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         (0..NUM_TRANS_IN_UNIT * 10).for_each(|i| {
//             assert_eq!(
//                 tx.body_raw_mut()[i],
//                 AdvancedDriveDuty::to_duty(&drives[i], cycles[i])
//             );
//         });
//         assert_eq!(tx.num_bodies, 10);

//         op.pack(&mut tx).unwrap();
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(!tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         assert_eq!(tx.num_bodies, 0);

//         op.init();
//         assert!(!op.is_finished());
//     }

//     #[test]
//     fn advanced_phase_gain() {
//         let mut tx = TxDatagram::new(&[
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//         ]);

//         let mut rng = rand::thread_rng();

//         let drives = (0..NUM_TRANS_IN_UNIT * 10)
//             .map(|_| Drive {
//                 amp: 0.0,
//                 phase: rng.gen_range(0.0..1.0),
//             })
//             .collect::<Vec<_>>();
//         let cycles = (0..NUM_TRANS_IN_UNIT * 10)
//             .map(|_| rng.gen_range(2u16..0xFFFFu16))
//             .collect::<Vec<_>>();

//         let mut op = GainAdvancedPhase::new(drives.clone(), || cycles.clone());
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
//         assert!(!tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         (0..NUM_TRANS_IN_UNIT * 10).for_each(|i| {
//             assert_eq!(
//                 tx.body_raw_mut()[i],
//                 AdvancedDrivePhase::to_phase(&drives[i], cycles[i])
//             );
//         });
//         assert_eq!(tx.num_bodies, 10);

//         op.pack(&mut tx).unwrap();
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(!tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         assert_eq!(tx.num_bodies, 0);

//         op.init();
//         assert!(!op.is_finished());
//     }

//     #[test]
//     fn advanced_duty_gain() {
//         let mut tx = TxDatagram::new(&[
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//             NUM_TRANS_IN_UNIT,
//         ]);

//         let mut rng = rand::thread_rng();

//         let drives = (0..NUM_TRANS_IN_UNIT * 10)
//             .map(|_| Drive {
//                 amp: 0.0,
//                 phase: rng.gen_range(0.0..1.0),
//             })
//             .collect::<Vec<_>>();
//         let cycles = (0..NUM_TRANS_IN_UNIT * 10)
//             .map(|_| rng.gen_range(2u16..0xFFFFu16))
//             .collect::<Vec<_>>();

//         let mut op = GainAdvancedDuty::new(drives.clone(), || cycles.clone());
//         op.init();
//         assert!(!op.is_finished());

//         op.pack(&mut tx).unwrap();
//         assert!(op.is_finished());
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
//         assert!(!tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         (0..NUM_TRANS_IN_UNIT * 10).for_each(|i| {
//             assert_eq!(
//                 tx.body_raw_mut()[i],
//                 AdvancedDriveDuty::to_duty(&drives[i], cycles[i])
//             );
//         });
//         assert_eq!(tx.num_bodies, 10);

//         op.pack(&mut tx).unwrap();
//         assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
//         assert!(!tx
//             .header()
//             .fpga_flag
//             .contains(FPGAControlFlags::LEGACY_MODE));
//         assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
//         assert_eq!(tx.num_bodies, 0);

//         op.init();
//         assert!(!op.is_finished());
//     }
// }
