/*
 * File: datagram.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::cpu::{Body, GlobalHeader};

#[derive(Clone)]
pub struct TxDatagram {
    data: Vec<u8>,
    size: usize,
    pub num_bodies: usize,
}

impl TxDatagram {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![
                0x00;
                std::mem::size_of::<GlobalHeader>() + std::mem::size_of::<Body>() * size
            ],
            size,
            num_bodies: size,
        }
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<GlobalHeader>() + std::mem::size_of::<Body>() * self.num_bodies
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn header(&self) -> &GlobalHeader {
        unsafe {
            (self.data.as_ptr() as *const GlobalHeader)
                .as_ref()
                .unwrap()
        }
    }

    pub fn header_mut(&mut self) -> &mut GlobalHeader {
        unsafe {
            (self.data.as_mut_ptr() as *mut GlobalHeader)
                .as_mut()
                .unwrap()
        }
    }

    pub fn body(&self) -> &[Body] {
        unsafe {
            let ptr = self.data.as_ptr().add(std::mem::size_of::<GlobalHeader>()) as *const Body;
            std::slice::from_raw_parts(ptr, self.size)
        }
    }

    pub fn body_mut(&mut self) -> &mut [Body] {
        unsafe {
            let ptr = self
                .data
                .as_mut_ptr()
                .add(std::mem::size_of::<GlobalHeader>()) as *mut Body;
            std::slice::from_raw_parts_mut(ptr, self.size)
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
    pub fn new() -> Self {
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

    pub fn copy_from(&mut self, src: &RxDatagram) {
        self.data.copy_from_slice(&src.data);
    }

    pub fn messages(&self) -> &[RxMessage] {
        &self.data
    }

    pub fn messages_mut(&mut self) -> &mut [RxMessage] {
        &mut self.data
    }
}

pub fn is_msg_processed(msg_id: u8, rx: &RxDatagram) -> bool {
    rx.data.iter().all(|msg| msg.msg_id == msg_id)
}
