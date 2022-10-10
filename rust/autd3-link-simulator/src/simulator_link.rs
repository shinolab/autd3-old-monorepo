/*
 * File: debug_link.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::net::UdpSocket;

use autd3_core::{
    geometry::{Geometry, Transducer},
    link::Link,
    CPUControlFlags, FPGAControlFlags, RxDatagram, TxDatagram, MSG_EMU_GEOMETRY_SET,
    MSG_RD_CPU_VERSION, MSG_RD_FPGA_FUNCTION, MSG_RD_FPGA_VERSION,
};

pub struct Simulator {
    port: u16,
    socket: Option<UdpSocket>,
    last_msg_id: u8,
}

impl Simulator {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            socket: None,
            last_msg_id: 0,
        }
    }
}

impl Link for Simulator {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        let mut geometry_buf = TxDatagram::new(geometry.num_devices());
        geometry_buf.num_bodies = geometry.num_devices();

        let header = geometry_buf.header_mut();
        header.msg_id = MSG_EMU_GEOMETRY_SET;
        header.cpu_flag = CPUControlFlags::NONE;
        header.fpga_flag = FPGAControlFlags::NONE;
        header.size = 0x00;

        geometry
            .devices()
            .iter()
            .zip(geometry_buf.body_mut())
            .for_each(|(device, body)| {
                let origin = device.transducers()[0].position();
                let right = device.transducers()[0].x_direction();
                let up = device.transducers()[0].y_direction();

                let dst = body.data.as_mut_ptr() as *mut f32;
                unsafe {
                    dst.add(0).write(origin.x as f32);
                    dst.add(1).write(origin.y as f32);
                    dst.add(2).write(origin.z as f32);
                    dst.add(3).write(right.x as f32);
                    dst.add(4).write(right.y as f32);
                    dst.add(5).write(right.z as f32);
                    dst.add(6).write(up.x as f32);
                    dst.add(7).write(up.y as f32);
                    dst.add(8).write(up.z as f32);
                }
            });

        let socket = UdpSocket::bind("0.0.0.0:8080")?;
        let remote_addr = format!("127.0.0.1:{}", self.port);
        socket.connect(remote_addr)?;
        socket.send(geometry_buf.data())?;
        self.socket = Some(socket);
        Ok(())
    }

    fn close(&mut self) -> anyhow::Result<()> {
        self.socket = None;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> anyhow::Result<bool> {
        if let Some(socket) = &self.socket {
            socket.try_clone()?.send(tx.data())?;
            self.last_msg_id = tx.header().msg_id;
        }
        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> anyhow::Result<bool> {
        for r in rx.messages_mut() {
            r.msg_id = self.last_msg_id;
        }

        let mut set = |value: u8| {
            for r in rx.messages_mut() {
                r.ack = value;
            }
        };

        match self.last_msg_id {
            MSG_RD_CPU_VERSION | MSG_RD_FPGA_FUNCTION | MSG_RD_FPGA_VERSION => set(0xFF),
            _ => (),
        }
        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.socket.is_some()
    }
}
