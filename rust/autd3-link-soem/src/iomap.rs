/*
 * File: iomap.rs
 * Project: src
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    GlobalHeader, RxDatagram, RxMessage, TxDatagram, BODY_SIZE, EC_INPUT_FRAME_SIZE,
    EC_OUTPUT_FRAME_SIZE, NUM_TRANS_IN_UNIT,
};

pub struct IOMap {
    buf: Vec<u8>,
    num_devices: usize,
}

impl IOMap {
    pub fn new(num_devices: usize) -> Self {
        Self {
            buf: vec![0x00; (EC_OUTPUT_FRAME_SIZE + EC_INPUT_FRAME_SIZE) * num_devices],
            num_devices,
        }
    }

    fn header(&mut self, i: usize) -> *mut GlobalHeader {
        unsafe {
            self.buf
                .as_mut_ptr()
                .add(EC_OUTPUT_FRAME_SIZE * i + BODY_SIZE) as *mut _
        }
    }

    fn body(&mut self, i: usize) -> *mut u16 {
        unsafe { self.buf.as_mut_ptr().add(EC_OUTPUT_FRAME_SIZE * i) as *mut _ }
    }

    pub fn data(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    pub fn input(&self) -> RxDatagram {
        let mut rx = RxDatagram::new(self.num_devices);
        unsafe {
            let data = std::slice::from_raw_parts(
                self.buf
                    .as_ptr()
                    .add(EC_OUTPUT_FRAME_SIZE * self.num_devices)
                    as *const RxMessage,
                self.num_devices,
            );
            rx.messages_mut()
                .iter_mut()
                .zip(data.iter())
                .for_each(|(d, s)| d.clone_from(s));
        }
        rx
    }

    pub fn copy_from(&mut self, tx: TxDatagram) {
        tx.body()
            .iter()
            .take(tx.num_bodies)
            .enumerate()
            .for_each(|(i, b)| unsafe {
                std::ptr::copy_nonoverlapping(b.data.as_ptr(), self.body(i), NUM_TRANS_IN_UNIT);
            });
        (0..self.num_devices).for_each(|i| unsafe {
            std::ptr::copy_nonoverlapping(tx.header() as *const _, self.header(i), 1);
        });
    }
}
