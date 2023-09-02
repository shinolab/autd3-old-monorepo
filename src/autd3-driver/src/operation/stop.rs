/*
 * File: stop.rs
 * Project: operation
 * Created Date: 01/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    defined::Drive,
    error::AUTDInternalError,
    fpga::AdvancedDriveDuty,
    geometry::{Device, Transducer},
    operation::{GainControlFlags, TypeTag},
};

use super::Operation;

#[derive(Default)]
pub struct StopOp {
    remains: HashMap<usize, usize>,
}

impl<T: Transducer> Operation<T> for StopOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        tx[0] = TypeTag::Gain as u8;
        tx[1] = GainControlFlags::DUTY.bits();

        assert!(tx.len() > 2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>());

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                (&mut tx[2..]).as_mut_ptr() as *mut AdvancedDriveDuty,
                device.num_transducers(),
            );
            dst.iter_mut()
                .zip(device.iter().map(|tr| tr.cycle()))
                .for_each(|(d, c)| d.set(&Drive { amp: 0., phase: 0. }, c));
        }

        Ok(2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>())
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>()
    }

    fn init(&mut self, devices: &[&Device<T>]) -> Result<(), AUTDInternalError> {
        self.remains = devices.iter().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains.insert(device.idx(), 0);
    }
}
