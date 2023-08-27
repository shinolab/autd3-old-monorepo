/*
 * File: datagram.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::ops::{Deref, DerefMut};

use crate::{
    cpu::{Body, GlobalHeader, LegacyPhaseFull, LegacyPhaseHalf},
    fpga::{AdvancedDriveDuty, AdvancedDrivePhase, LegacyDrive},
};

#[derive(Clone)]
pub struct TxDatagram {
    data: Vec<u8>,
    body_pointer: Vec<usize>,
    device_map: Vec<usize>,
    pub num_bodies: usize,
}

impl TxDatagram {
    pub fn new(device_map: &[usize]) -> Self {
        let device_map = device_map.to_vec();
        let num_bodies = device_map.len();
        let head = &[0usize];
        let body_pointer = head
            .iter()
            .chain(device_map.iter())
            .scan(0, |state, tr_num| {
                *state += std::mem::size_of::<u16>() * tr_num;
                Some(*state)
            })
            .collect::<Vec<_>>();
        Self {
            data: vec![0x00; std::mem::size_of::<GlobalHeader>() + body_pointer.last().unwrap()],
            body_pointer,
            device_map,
            num_bodies,
        }
    }

    pub fn num_devices(&self) -> usize {
        self.body_pointer.len() - 1
    }

    pub fn num_transducers(&self) -> usize {
        self.body_pointer[self.num_bodies] / std::mem::size_of::<u16>()
    }

    pub fn dev_map(&self) -> &[usize] {
        &self.device_map
    }

    pub fn transmitting_size(&self) -> usize {
        std::mem::size_of::<GlobalHeader>() + self.body_pointer[self.num_bodies]
    }

    pub fn body_size(&self) -> usize {
        self.body_pointer[self.num_bodies]
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn header(&self) -> &GlobalHeader {
        unsafe { &*(self.data.as_ptr() as *const GlobalHeader) }
    }

    pub fn header_mut(&mut self) -> &mut GlobalHeader {
        unsafe { &mut *(self.data.as_mut_ptr() as *mut GlobalHeader) }
    }

    pub fn body_raw_mut(&mut self) -> &mut [u16] {
        let len =
            (self.data.len() - std::mem::size_of::<GlobalHeader>()) / std::mem::size_of::<u16>();
        unsafe {
            &mut *std::ptr::slice_from_raw_parts_mut(
                self.data
                    .as_mut_ptr()
                    .add(std::mem::size_of::<GlobalHeader>()) as *mut u16,
                len,
            )
        }
    }

    pub fn legacy_drives_mut(&mut self) -> &mut [LegacyDrive] {
        let len =
            (self.data.len() - std::mem::size_of::<GlobalHeader>()) / std::mem::size_of::<u16>();
        unsafe {
            &mut *std::ptr::slice_from_raw_parts_mut(
                self.data
                    .as_mut_ptr()
                    .add(std::mem::size_of::<GlobalHeader>()) as *mut LegacyDrive,
                len,
            )
        }
    }

    pub fn legacy_phase_full_mut<const N: usize>(&mut self) -> &mut [LegacyPhaseFull<N>] {
        let len =
            (self.data.len() - std::mem::size_of::<GlobalHeader>()) / std::mem::size_of::<u16>();
        unsafe {
            &mut *std::ptr::slice_from_raw_parts_mut(
                self.data
                    .as_mut_ptr()
                    .add(std::mem::size_of::<GlobalHeader>())
                    as *mut LegacyPhaseFull<N>,
                len,
            )
        }
    }

    pub fn legacy_phase_half_mut<const N: usize>(&mut self) -> &mut [LegacyPhaseHalf<N>] {
        let len =
            (self.data.len() - std::mem::size_of::<GlobalHeader>()) / std::mem::size_of::<u16>();
        unsafe {
            &mut *std::ptr::slice_from_raw_parts_mut(
                self.data
                    .as_mut_ptr()
                    .add(std::mem::size_of::<GlobalHeader>())
                    as *mut LegacyPhaseHalf<N>,
                len,
            )
        }
    }

    pub fn duties_mut(&mut self) -> &mut [AdvancedDriveDuty] {
        let len =
            (self.data.len() - std::mem::size_of::<GlobalHeader>()) / std::mem::size_of::<u16>();
        unsafe {
            &mut *std::ptr::slice_from_raw_parts_mut(
                self.data
                    .as_mut_ptr()
                    .add(std::mem::size_of::<GlobalHeader>())
                    as *mut AdvancedDriveDuty,
                len,
            )
        }
    }

    pub fn phases_mut(&mut self) -> &mut [AdvancedDrivePhase] {
        let len =
            (self.data.len() - std::mem::size_of::<GlobalHeader>()) / std::mem::size_of::<u16>();
        unsafe {
            &mut *std::ptr::slice_from_raw_parts_mut(
                self.data
                    .as_mut_ptr()
                    .add(std::mem::size_of::<GlobalHeader>())
                    as *mut AdvancedDrivePhase,
                len,
            )
        }
    }

    pub fn body(&self, idx: usize) -> &Body<[u16]> {
        unsafe {
            let ptr = self
                .data
                .as_ptr()
                .add(std::mem::size_of::<GlobalHeader>() + self.body_pointer[idx]);
            let len = self.device_map[idx];
            &*(std::ptr::slice_from_raw_parts(ptr as *const u16, len) as *const Body<[u16]>)
        }
    }

    pub fn body_mut(&mut self, idx: usize) -> &mut Body<[u16]> {
        unsafe {
            let ptr = self
                .data
                .as_mut_ptr()
                .add(std::mem::size_of::<GlobalHeader>() + self.body_pointer[idx]);
            let len = self.device_map[idx];
            &mut *(std::ptr::slice_from_raw_parts_mut(ptr as *mut u16, len) as *mut Body<[u16]>)
        }
    }

    pub fn copy_from(&mut self, src: &TxDatagram) {
        self.data.copy_from_slice(&src.data);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RxMessage {
    pub ack: u8,
    pub msg_id: u8,
}

impl RxMessage {
    pub const fn new() -> Self {
        Self { ack: 0, msg_id: 0 }
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

    pub fn is_msg_processed(&self, msg_id: u8) -> bool {
        self.data.iter().all(|msg| msg.msg_id == msg_id)
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
    fn tx_datagram() {
        let device_map = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut tx = TxDatagram::new(&device_map);

        assert_eq!(tx.num_devices(), 10);
        assert_eq!(tx.num_transducers(), 55);
        assert_eq!(tx.transmitting_size(), 128 + size_of::<u16>() * 55);

        tx.num_bodies = 5;
        assert_eq!(tx.num_devices(), 10);
        assert_eq!(tx.num_transducers(), 15);
        assert_eq!(tx.transmitting_size(), 128 + size_of::<u16>() * 15);

        assert_eq!(tx.data().as_ptr(), tx.header() as *const _ as *const u8);
        unsafe {
            assert_eq!(
                tx.data().as_ptr().add(128),
                tx.body_raw_mut().as_ptr() as *const u8
            );
            let mut cursor = tx.data().as_ptr().add(128);
            device_map.iter().enumerate().for_each(|(i, dev)| {
                assert_eq!(cursor, tx.body(i) as *const _ as *const u8);
                cursor = cursor.add(size_of::<u16>() * dev);
            });
        }
    }

    #[test]
    fn rx_datagram() {
        assert_eq!(size_of::<RxMessage>(), 2);

        let mut rx = RxDatagram::new(10);

        assert!(!rx.is_msg_processed(1));

        rx[0].msg_id = 1;
        assert!(!rx.is_msg_processed(1));

        rx.iter_mut().for_each(|msg| {
            msg.msg_id = 1;
        });
        assert!(rx.is_msg_processed(1));
        assert!(!rx.is_msg_processed(2));
    }
}
