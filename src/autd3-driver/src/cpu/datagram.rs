/*
 * File: rx_message.rs
 * Project: cpu
 * Created Date: 29/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::ops::{Deref, DerefMut};

use crate::cpu::{Header, EC_OUTPUT_FRAME_SIZE};

#[derive(Clone)]
pub struct TxDatagram {
    data: Vec<u8>,
    num_devices: usize,
}

impl TxDatagram {
    pub fn new(num_devices: usize) -> Self {
        Self {
            data: vec![0x00; (EC_OUTPUT_FRAME_SIZE) * num_devices],
            num_devices,
        }
    }

    pub fn all_data(&self) -> &[u8] {
        &self.data
    }

    pub fn data(&self, i: usize) -> &[u8] {
        &self.data[i * EC_OUTPUT_FRAME_SIZE..(i + 1) * EC_OUTPUT_FRAME_SIZE]
    }

    pub fn headers(&self) -> impl Iterator<Item = &Header> {
        (0..self.num_devices).map(|i| self.header(i))
    }

    pub fn header(&self, i: usize) -> &Header {
        unsafe {
            (self.data[i * EC_OUTPUT_FRAME_SIZE..].as_ptr() as *const Header)
                .as_ref()
                .unwrap()
        }
    }

    pub fn header_mut(&mut self, i: usize) -> &mut Header {
        unsafe {
            (self.data[i * EC_OUTPUT_FRAME_SIZE..].as_mut_ptr() as *mut Header)
                .as_mut()
                .unwrap()
        }
    }

    pub fn payload(&self, i: usize) -> &[u8] {
        &self.data[i * EC_OUTPUT_FRAME_SIZE + std::mem::size_of::<Header>()
            ..(i + 1) * EC_OUTPUT_FRAME_SIZE]
    }

    pub fn payload_mut(&mut self, i: usize) -> &mut [u8] {
        &mut self.data[i * EC_OUTPUT_FRAME_SIZE + std::mem::size_of::<Header>()
            ..(i + 1) * EC_OUTPUT_FRAME_SIZE]
    }

    pub fn payloads(&self) -> impl Iterator<Item = &[u8]> {
        (0..self.num_devices).map(|i| self.payload(i))
    }

    pub fn copy_from(&mut self, src: &TxDatagram) {
        self.data.copy_from_slice(&src.data);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RxMessage {
    pub data: u8,
    pub ack: u8,
}

impl RxMessage {
    pub const fn new() -> Self {
        Self { data: 0, ack: 0 }
    }
}

impl Default for RxMessage {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RxDatagram {
    data: Vec<RxMessage>,
}

impl RxDatagram {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![RxMessage::default(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn copy_from(&mut self, src: &RxDatagram) {
        self.data.copy_from_slice(&src.data);
    }

    pub fn copy_from_iter<I: IntoIterator<Item = RxMessage>>(&mut self, src: I) {
        self.data.iter_mut().zip(src).for_each(|(dst, src)| {
            *dst = src;
        });
    }
}

impl Deref for RxDatagram {
    type Target = [RxMessage];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for RxDatagram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn rx_datagram() {
        assert_eq!(size_of::<RxMessage>(), 2);
    }
}
