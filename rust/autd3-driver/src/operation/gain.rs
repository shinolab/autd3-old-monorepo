/*
 * File: gain.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{CPUControlFlags, Drive, DriverError, FPGAControlFlags, TxDatagram};

pub trait GainOp: Operation {
    fn new<F: Fn() -> Vec<u16>>(drives: Vec<Drive>, cycles_fn: F) -> Self;
}

pub struct GainLegacy {
    sent: bool,
    drives: Vec<Drive>,
}

impl GainOp for GainLegacy {
    fn new<F: Fn() -> Vec<u16>>(drives: Vec<Drive>, _: F) -> Self {
        Self {
            sent: false,
            drives,
        }
    }
}

impl Operation for GainLegacy {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

        tx.header_mut()
            .fpga_flag
            .set(FPGAControlFlags::LEGACY_MODE, true);
        tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

        tx.num_bodies = 0;

        if self.sent {
            return Ok(());
        }

        tx.num_bodies = tx.num_devices();
        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            });
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);

        tx.legacy_drives_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .for_each(|(d, s)| d.set(s));

        self.sent = true;
        Ok(())
    }

    fn init(&mut self) {
        self.sent = false;
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}

pub struct GainAdvanced {
    phase_sent: bool,
    duty_sent: bool,
    drives: Vec<Drive>,
    cycles: Vec<u16>,
}

impl GainOp for GainAdvanced {
    fn new<F: Fn() -> Vec<u16>>(drives: Vec<Drive>, cycles_fn: F) -> Self {
        Self {
            phase_sent: false,
            duty_sent: false,
            drives,
            cycles: cycles_fn(),
        }
    }
}

impl GainAdvanced {
    fn pack_duty(&self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.num_bodies = tx.num_devices();

        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            });
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

        tx.duties_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        Ok(())
    }

    fn pack_phase(&self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.num_bodies = tx.num_devices();

        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            });
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

        tx.phases_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        Ok(())
    }
}

impl Operation for GainAdvanced {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

        tx.header_mut()
            .fpga_flag
            .remove(FPGAControlFlags::LEGACY_MODE);
        tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        if !self.phase_sent {
            self.pack_phase(tx)?;
            self.phase_sent = true;
            return Ok(());
        }

        self.pack_duty(tx)?;
        self.duty_sent = true;

        Ok(())
    }

    fn init(&mut self) {
        self.phase_sent = false;
        self.duty_sent = false;
    }

    fn is_finished(&self) -> bool {
        self.phase_sent && self.duty_sent
    }
}

pub struct GainAdvancedPhase {
    sent: bool,
    drives: Vec<Drive>,
    cycles: Vec<u16>,
}

impl GainOp for GainAdvancedPhase {
    fn new<F: Fn() -> Vec<u16>>(drives: Vec<Drive>, cycles_fn: F) -> Self {
        Self {
            sent: false,
            drives,
            cycles: cycles_fn(),
        }
    }
}

impl Operation for GainAdvancedPhase {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

        tx.header_mut()
            .fpga_flag
            .remove(FPGAControlFlags::LEGACY_MODE);
        tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        tx.num_bodies = tx.num_devices();

        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            });
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::IS_DUTY);

        tx.phases_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        self.sent = true;
        Ok(())
    }

    fn init(&mut self) {
        self.sent = false;
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}

pub struct GainAdvancedDuty {
    sent: bool,
    drives: Vec<Drive>,
    cycles: Vec<u16>,
}

impl GainOp for GainAdvancedDuty {
    fn new<F: Fn() -> Vec<u16>>(drives: Vec<Drive>, cycles_fn: F) -> Self {
        Self {
            sent: false,
            drives,
            cycles: cycles_fn(),
        }
    }
}

impl Operation for GainAdvancedDuty {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<(), DriverError> {
        tx.header_mut().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
        tx.header_mut().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

        tx.header_mut()
            .fpga_flag
            .remove(FPGAControlFlags::LEGACY_MODE);
        tx.header_mut().fpga_flag.remove(FPGAControlFlags::STM_MODE);

        tx.num_bodies = 0;

        if self.is_finished() {
            return Ok(());
        }

        tx.num_bodies = tx.num_devices();

        if self.drives.len() != tx.num_transducers() {
            return Err(DriverError::NumberOfTransducerMismatch {
                a: tx.num_transducers(),
                b: self.drives.len(),
            });
        }

        tx.header_mut()
            .cpu_flag
            .set(CPUControlFlags::WRITE_BODY, true);
        tx.header_mut().cpu_flag.set(CPUControlFlags::IS_DUTY, true);

        tx.duties_mut()
            .iter_mut()
            .zip(self.drives.iter())
            .zip(self.cycles.iter())
            .for_each(|((d, s), &c)| d.set(s, c));

        self.sent = true;
        Ok(())
    }

    fn init(&mut self) {
        self.sent = false;
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}

#[cfg(test)]
mod test {
    use rand::prelude::*;

    use crate::{AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive};

    use super::*;

    const NUM_TRANS_IN_UNIT: usize = 249;

    #[test]
    fn legacy_gain() {
        let mut tx = TxDatagram::new(&[
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
        ]);

        let mut rng = rand::thread_rng();

        let drives = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| Drive {
                amp: rng.gen_range(0.0..1.0),
                phase: rng.gen_range(0.0..1.0),
            })
            .collect::<Vec<_>>();

        let mut op = GainLegacy::new(drives.clone(), || vec![4096; NUM_TRANS_IN_UNIT * 10]);
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());

        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        for (&d, drive) in tx.body_raw_mut().iter().zip(drives.iter()) {
            assert_eq!((d & 0xFF) as u8, LegacyDrive::to_phase(drive));
            assert_eq!((d >> 8) as u8, LegacyDrive::to_duty(drive));
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert_eq!(tx.num_bodies, 0);

        op.init();
        assert!(!op.is_finished());
    }

    #[test]
    fn advanced_gain() {
        let mut tx = TxDatagram::new(&[
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
        ]);

        let mut rng = rand::thread_rng();

        let drives = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| Drive {
                amp: rng.gen_range(0.0..1.0),
                phase: rng.gen_range(0.0..1.0),
            })
            .collect::<Vec<_>>();
        let cycles = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| rng.gen_range(2u16..0xFFFFu16))
            .collect::<Vec<_>>();
        let mut op = GainAdvanced::new(drives.clone(), || cycles.clone());
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(!op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        for i in 0..NUM_TRANS_IN_UNIT * 10 {
            assert_eq!(
                tx.body_raw_mut()[i],
                AdvancedDrivePhase::to_phase(&drives[i], cycles[i])
            );
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        for i in 0..NUM_TRANS_IN_UNIT * 10 {
            assert_eq!(
                tx.body_raw_mut()[i],
                AdvancedDriveDuty::to_duty(&drives[i], cycles[i])
            );
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert_eq!(tx.num_bodies, 0);

        op.init();
        assert!(!op.is_finished());
    }

    #[test]
    fn advanced_phase_gain() {
        let mut tx = TxDatagram::new(&[
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
        ]);

        let mut rng = rand::thread_rng();

        let drives = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| Drive {
                amp: 0.0,
                phase: rng.gen_range(0.0..1.0),
            })
            .collect::<Vec<_>>();
        let cycles = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| rng.gen_range(2u16..0xFFFFu16))
            .collect::<Vec<_>>();

        let mut op = GainAdvancedPhase::new(drives.clone(), || cycles.clone());
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        for i in 0..NUM_TRANS_IN_UNIT * 10 {
            assert_eq!(
                tx.body_raw_mut()[i],
                AdvancedDrivePhase::to_phase(&drives[i], cycles[i])
            );
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert_eq!(tx.num_bodies, 0);

        op.init();
        assert!(!op.is_finished());
    }

    #[test]
    fn advanced_duty_gain() {
        let mut tx = TxDatagram::new(&[
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
            NUM_TRANS_IN_UNIT,
        ]);

        let mut rng = rand::thread_rng();

        let drives = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| Drive {
                amp: 0.0,
                phase: rng.gen_range(0.0..1.0),
            })
            .collect::<Vec<_>>();
        let cycles = (0..NUM_TRANS_IN_UNIT * 10)
            .map(|_| rng.gen_range(2u16..0xFFFFu16))
            .collect::<Vec<_>>();

        let mut op = GainAdvancedDuty::new(drives.clone(), || cycles.clone());
        op.init();
        assert!(!op.is_finished());

        op.pack(&mut tx).unwrap();
        assert!(op.is_finished());
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(tx.header().cpu_flag.contains(CPUControlFlags::IS_DUTY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        for i in 0..NUM_TRANS_IN_UNIT * 10 {
            assert_eq!(
                tx.body_raw_mut()[i],
                AdvancedDriveDuty::to_duty(&drives[i], cycles[i])
            );
        }
        assert_eq!(tx.num_bodies, 10);

        op.pack(&mut tx).unwrap();
        assert!(!tx.header().cpu_flag.contains(CPUControlFlags::WRITE_BODY));
        assert!(!tx
            .header()
            .fpga_flag
            .contains(FPGAControlFlags::LEGACY_MODE));
        assert!(!tx.header().fpga_flag.contains(FPGAControlFlags::STM_MODE));
        assert_eq!(tx.num_bodies, 0);

        op.init();
        assert!(!op.is_finished());
    }
}
