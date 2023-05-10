/*
 * File: debug_link.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::{Link, LinkBuilder},
    CPUControlFlags, FPGAControlFlags, RxDatagram, RxMessage, TxDatagram, HEADER_SIZE,
    MSG_SIMULATOR_CLOSE, MSG_SIMULATOR_INIT,
};

use crate::native_methods::*;

pub struct Simulator {
    input_offset: usize,
    is_open: bool,
    timeout: Duration,
}

pub struct SimulatorBuilder {
    timeout: Duration,
}

impl Simulator {
    fn with_timeout(timeout: Duration) -> Self {
        Self {
            input_offset: 0,
            is_open: false,
            timeout,
        }
    }

    pub fn builder() -> SimulatorBuilder {
        SimulatorBuilder::new()
    }
}

impl SimulatorBuilder {
    fn new() -> Self {
        Self {
            timeout: Duration::ZERO,
        }
    }
}

impl LinkBuilder for SimulatorBuilder {
    type L = Simulator;

    fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn build(self) -> Self::L {
        Simulator::with_timeout(self.timeout)
    }
}

unsafe impl Send for Simulator {}

impl Link for Simulator {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        if self.is_open() {
            return Ok(());
        }

        self.input_offset = HEADER_SIZE + geometry.num_transducers() * std::mem::size_of::<u16>();

        let geometry_size = std::mem::size_of::<u8>()
            + std::mem::size_of::<u32>()
            + std::mem::size_of::<u32>() * geometry.num_devices()
            + geometry.num_transducers() * std::mem::size_of::<f32>() * 7;

        unsafe {
            if !shmem_create() {
                return Err(AUTDInternalError::LinkError(
                    "Shared memory open failed".to_string(),
                ));
            }

            let mut geometry_buf: Vec<u8> = vec![0x00; geometry_size];
            let mut cursor: *mut u8 = geometry_buf.as_mut_ptr();

            std::ptr::write(cursor, MSG_SIMULATOR_INIT);
            cursor = cursor.add(1);

            std::ptr::write(cursor as *mut u32, geometry.num_devices() as u32);
            cursor = cursor.add(std::mem::size_of::<u32>());

            let mut i = 0;
            let mut c = 0;
            (0..geometry.num_devices()).for_each(|dev| {
                c += geometry.device_map()[dev];

                std::ptr::write(cursor as *mut u32, geometry.device_map()[dev] as u32);
                cursor = cursor.add(std::mem::size_of::<u32>());

                let mut p = cursor as *mut f32;
                (i..c).for_each(|id| {
                    let tr = &geometry[id];
                    let origin = tr.position();
                    let rot = tr.rotation();
                    std::ptr::write(p, origin.x as _);
                    p = p.add(1);
                    std::ptr::write(p, origin.y as _);
                    p = p.add(1);
                    std::ptr::write(p, origin.z as _);
                    p = p.add(1);
                    std::ptr::write(p, rot.w as _);
                    p = p.add(1);
                    std::ptr::write(p, rot.i as _);
                    p = p.add(1);
                    std::ptr::write(p, rot.j as _);
                    p = p.add(1);
                    std::ptr::write(p, rot.k as _);
                    p = p.add(1);
                    cursor = cursor.add(std::mem::size_of::<f32>() * 7);
                });
                i = c;
            });
            shmem_copy_to(geometry_buf.as_ptr(), geometry_size);
        }

        unsafe {
            for _ in 0..20 {
                std::thread::sleep(Duration::from_millis(100));
                let mut msg: u8 = 0;
                shmem_copy_from(&mut msg as *mut u8, 0, 1);
                if msg != MSG_SIMULATOR_INIT {
                    self.is_open = true;
                    return Ok(());
                }
            }
        }

        Err(AUTDInternalError::LinkError(
            "Simulator is not responding".to_string(),
        ))
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if !self.is_open() {
            return Ok(());
        }

        let mut geometry_buf = TxDatagram::new(&[0]);
        let header = geometry_buf.header_mut();
        header.msg_id = MSG_SIMULATOR_CLOSE;
        header.cpu_flag = CPUControlFlags::NONE;
        header.fpga_flag = FPGAControlFlags::NONE;
        header.size = 0x00;
        geometry_buf.num_bodies = 0;

        self.send(&geometry_buf)?;

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open() {
            return Ok(false);
        }

        unsafe {
            shmem_copy_to(tx.data().as_ptr(), tx.transmitting_size());
        }

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open() {
            return Ok(false);
        }

        unsafe {
            shmem_copy_from(
                rx.messages_mut().as_mut_ptr() as *mut u8,
                self.input_offset,
                rx.messages().len() * std::mem::size_of::<RxMessage>(),
            );
        }

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
