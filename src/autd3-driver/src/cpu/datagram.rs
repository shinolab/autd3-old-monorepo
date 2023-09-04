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

use crate::{
    cpu::{Header, EC_OUTPUT_FRAME_SIZE},
    geometry::{Geometry, Transducer},
};

#[derive(Clone)]
pub struct TxDatagram {
    data: Vec<u8>,
    data_pointer: Vec<usize>,
    device_map: Vec<usize>,
}

impl TxDatagram {
    pub fn new<T: Transducer>(geometry: &Geometry<T>) -> Self {
        let device_map = geometry
            .iter()
            .map(|dev| dev.num_transducers())
            .collect::<Vec<_>>();
        let data_pointer = [0usize]
            .iter()
            .chain(device_map.iter())
            .scan(0, |state, tr_num| {
                *state += std::mem::size_of::<u16>() * tr_num;
                Some(*state)
            })
            .collect::<Vec<_>>();
        Self {
            data: vec![0x00; (EC_OUTPUT_FRAME_SIZE) * device_map.len()],
            data_pointer,
            device_map,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn headers(&self) -> impl Iterator<Item = &Header> {
        (0..self.device_map.len()).map(|i| self.header(i))
    }

    pub fn header(&self, i: usize) -> &Header {
        unsafe {
            (self.data[self.data_pointer[i]..].as_ptr() as *const Header)
                .as_ref()
                .unwrap()
        }
    }

    pub fn header_mut(&mut self, i: usize) -> &mut Header {
        unsafe {
            (self.data[self.data_pointer[i]..].as_mut_ptr() as *mut Header)
                .as_mut()
                .unwrap()
        }
    }

    pub fn payload(&self, i: usize) -> &[u8] {
        &self.data[self.data_pointer[i] + std::mem::size_of::<Header>()
            ..self.data_pointer[i] + EC_OUTPUT_FRAME_SIZE]
    }

    pub fn payload_mut(&mut self, i: usize) -> &mut [u8] {
        &mut self.data[self.data_pointer[i] + std::mem::size_of::<Header>()
            ..self.data_pointer[i] + EC_OUTPUT_FRAME_SIZE]
    }

    pub fn payloads(&self) -> impl Iterator<Item = &[u8]> {
        (0..self.device_map.len()).map(|i| self.payload(i))
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
        self.data
            .iter_mut()
            .zip(src.into_iter())
            .for_each(|(dst, src)| {
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
