/*
 * File: iomap.rs
 * Project: src
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{GlobalHeader, RxMessage, TxDatagram, EC_INPUT_FRAME_SIZE, HEADER_SIZE};

pub struct IOMap {
    buf: Vec<u8>,
    trans_num_prefix_sum: Vec<usize>,
    device_map: Vec<usize>,
}

impl IOMap {
    pub fn new(device_map: &[usize]) -> Self {
        let head = &[0usize];
        let trans_num_prefix_sum = device_map
            .iter()
            .scan(0, |state, tr_num| {
                *state += HEADER_SIZE + std::mem::size_of::<u16>() * tr_num;
                Some(*state)
            })
            .collect::<Vec<_>>();
        let trans_num_prefix_sum = head
            .iter()
            .chain(trans_num_prefix_sum.iter())
            .copied()
            .collect::<Vec<_>>();
        let device_map = device_map.to_vec();
        let size = trans_num_prefix_sum.last().unwrap() + device_map.len() * EC_INPUT_FRAME_SIZE;
        Self {
            buf: vec![0x00; size],
            trans_num_prefix_sum,
            device_map,
        }
    }

    fn header(&mut self, i: usize) -> *mut GlobalHeader {
        unsafe {
            self.buf
                .as_mut_ptr()
                .add(self.trans_num_prefix_sum[i] + std::mem::size_of::<u16>() * self.device_map[i])
                as *mut _
        }
    }

    fn body(&mut self, i: usize) -> *mut u16 {
        unsafe { self.buf.as_mut_ptr().add(self.trans_num_prefix_sum[i]) as *mut _ }
    }

    pub fn data(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    pub fn input(&self) -> *const RxMessage {
        unsafe {
            self.buf
                .as_ptr()
                .add(*self.trans_num_prefix_sum.last().unwrap()) as *const _
        }
    }

    pub fn copy_from(&mut self, tx: &TxDatagram) {
        (0..tx.num_bodies).for_each(|i| unsafe {
            std::ptr::copy_nonoverlapping(
                tx.body(i).data().as_ptr(),
                self.body(i),
                self.device_map[i],
            );
        });
        (0..self.device_map.len()).for_each(|i| unsafe {
            std::ptr::copy_nonoverlapping(tx.header() as *const _, self.header(i), 1);
        });
    }

    pub fn clear(&mut self) {
        self.buf.fill(0x00);
    }
}
