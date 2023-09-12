/*
 * File: iomap.rs
 * Project: src
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::cpu::{RxMessage, TxDatagram, EC_INPUT_FRAME_SIZE, EC_OUTPUT_FRAME_SIZE};

pub struct IOMap {
    buf: Vec<u8>,
    num_devices: usize,
}

impl IOMap {
    pub fn new(num_devices: usize) -> Self {
        let size = (EC_OUTPUT_FRAME_SIZE + EC_INPUT_FRAME_SIZE) * num_devices;
        Self {
            buf: vec![0x00; size],
            num_devices,
        }
    }

    pub fn data(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    pub fn input(&self) -> *const RxMessage {
        unsafe {
            self.buf
                .as_ptr()
                .add(self.num_devices * EC_OUTPUT_FRAME_SIZE) as *const _
        }
    }

    pub fn copy_from(&mut self, tx: &TxDatagram) {
        unsafe {
            std::ptr::copy_nonoverlapping(tx.all_data().as_ptr(), self.data(), tx.all_data().len());
        }
    }

    pub fn clear(&mut self) {
        self.buf.fill(0x00);
    }
}
