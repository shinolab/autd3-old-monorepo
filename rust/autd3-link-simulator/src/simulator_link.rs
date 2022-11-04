/*
 * File: debug_link.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    geometry::{Geometry, Transducer},
    link::Link,
    CPUControlFlags, FPGAControlFlags, RxDatagram, RxMessage, TxDatagram, BODY_SIZE,
    EC_INPUT_FRAME_SIZE, HEADER_SIZE, MSG_SIMULATOR_CLOSE, MSG_SIMULATOR_INIT,
};

use smem::*;

pub struct Simulator {
    num_devices: usize,
    smem: SMem,
    ptr: *mut u8,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            num_devices: 0,
            smem: SMem::new(),
            ptr: std::ptr::null_mut(),
        }
    }
}

unsafe impl Send for Simulator {}

impl Link for Simulator {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        if self.is_open() {
            return Ok(());
        }

        let size = HEADER_SIZE + geometry.num_devices() * (BODY_SIZE + EC_INPUT_FRAME_SIZE);

        self.smem.create("autd3_simulator_smem", size)?;
        self.ptr = self.smem.map();

        self.num_devices = geometry.num_devices();

        let mut geometry_buf = TxDatagram::new(geometry.num_devices());
        let header = geometry_buf.header_mut();
        header.msg_id = MSG_SIMULATOR_INIT;
        header.cpu_flag = CPUControlFlags::NONE;
        header.fpga_flag = FPGAControlFlags::NONE;
        header.size = geometry.num_devices() as _;

        geometry
            .devices()
            .iter()
            .zip(geometry_buf.body_mut())
            .for_each(|(device, body)| {
                let origin = device.transducers()[0].position();
                let rot = device.rotation();

                let dst = body.data.as_mut_ptr() as *mut f32;
                unsafe {
                    dst.add(0).write(origin.x as f32);
                    dst.add(1).write(origin.y as f32);
                    dst.add(2).write(origin.z as f32);
                    dst.add(3).write(rot.w as f32);
                    dst.add(4).write(rot.i as f32);
                    dst.add(5).write(rot.j as f32);
                    dst.add(6).write(rot.k as f32);
                }
            });

        self.send(&geometry_buf)?;
        unsafe {
            while std::ptr::read_volatile(self.ptr) == MSG_SIMULATOR_INIT {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
        Ok(())
    }

    fn close(&mut self) -> anyhow::Result<()> {
        if !self.is_open() {
            return Ok(());
        }

        let mut geometry_buf = TxDatagram::new(self.num_devices);
        let header = geometry_buf.header_mut();
        header.msg_id = MSG_SIMULATOR_CLOSE;
        header.cpu_flag = CPUControlFlags::NONE;
        header.fpga_flag = FPGAControlFlags::NONE;
        header.size = 0x00;
        geometry_buf.num_bodies = 0;

        self.smem.unmap();
        self.ptr = std::ptr::null_mut();

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> anyhow::Result<bool> {
        if !self.is_open() {
            return Ok(false);
        }

        unsafe {
            std::ptr::copy_nonoverlapping(tx.data().as_ptr(), self.ptr, tx.size());
        }

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> anyhow::Result<bool> {
        if !self.is_open() {
            return Ok(false);
        }

        unsafe {
            for i in 0..rx.messages().len() {
                let msg = self
                    .ptr
                    .add(HEADER_SIZE + self.num_devices * BODY_SIZE + i * EC_INPUT_FRAME_SIZE)
                    as *const RxMessage;
                rx.messages_mut()[i].ack = (*msg).ack;
                rx.messages_mut()[i].msg_id = (*msg).msg_id;
            }
        }

        Ok(true)
    }

    fn is_open(&self) -> bool {
        !self.ptr.is_null()
    }
}
