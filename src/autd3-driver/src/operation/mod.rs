/*
 * File: mod.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod clear;
// mod flag;
mod gain;
mod info;
// mod mod_delay;
mod modulation;
mod null;
mod silencer;
mod stop;
// mod stm_focus;
// mod stm_gain;
// mod sync;

pub use clear::*;
// pub use flag::*;
pub use gain::*;
pub use info::*;
// pub use mod_delay::*;
pub use modulation::*;
pub use null::*;
pub use silencer::*;
pub use stop::*;
// pub use stm_focus::*;
// pub use stm_gain::*;
// pub use sync::*;

use crate::{
    cpu::TxDatagram,
    error::AUTDInternalError,
    fpga::FPGAControlFlags,
    geometry::{Device, Transducer},
};

#[repr(u8)]
pub enum TypeTag {
    NONE = 0x00,
    Clear = 0x01,
    Sync = 0x02,
    FirmwareInfo = 0x03,
    Modulation = 0x10,
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
            0x10 => Self::Modulation,
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
    fn init(&mut self, devices: &[&Device<T>]) -> Result<(), AUTDInternalError>;
    fn required_size(&self, device: &Device<T>) -> usize;
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError>;
    fn commit(&mut self, device: &Device<T>);
    fn remains(&self, device: &Device<T>) -> usize;
}

impl<T: Transducer> Operation<T> for Box<dyn Operation<T>> {
    fn init(&mut self, devices: &[&Device<T>]) -> Result<(), AUTDInternalError> {
        self.as_mut().init(devices)
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
    pub fn is_finished<
        'a,
        T: Transducer + 'a,
        O1: Operation<T>,
        O2: Operation<T>,
        I: IntoIterator<Item = &'a Device<T>>,
    >(
        op1: &mut O1,
        op2: &mut O2,
        device_iter: I,
    ) -> bool {
        device_iter
            .into_iter()
            .all(|dev| op1.remains(dev) == 0 && op2.remains(dev) == 0)
    }

    pub fn init<
        'a,
        T: Transducer + 'a,
        O1: Operation<T>,
        O2: Operation<T>,
        I: IntoIterator<Item = &'a Device<T>>,
    >(
        op1: &mut O1,
        op2: &mut O2,
        device_iter: I,
    ) -> Result<(), AUTDInternalError> {
        let devices = device_iter.into_iter().collect::<Vec<_>>();
        op1.init(&devices)?;
        op2.init(&devices)
    }

    pub fn pack<
        'a,
        T: Transducer + 'a,
        O1: Operation<T>,
        O2: Operation<T>,
        I: IntoIterator<Item = &'a Device<T>>,
    >(
        op1: &mut O1,
        op2: &mut O2,
        device_iter: I,
        tx: &mut TxDatagram,
    ) -> Result<(), AUTDInternalError> {
        device_iter
            .into_iter()
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

                    let t = tx.body_mut(dev.idx());
                    assert!(t.len() > op1.required_size(dev));
                    let op1_size = op1.pack(dev, t)?;
                    op1.commit(dev);

                    if t.len() - op1_size > op2.required_size(dev) {
                        tx.header_mut(dev.idx()).slot_2_offset = op1_size as u16;
                        let t = tx.body_mut(dev.idx());
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

        op.pack(dev, tx.body_mut(dev.idx()))?;
        op.commit(dev);

        Ok(())
    }
}
