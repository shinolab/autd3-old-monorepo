/*
 * File: lib.rs
 * Project: src
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    io::{Read, Write},
    net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6, TcpStream},
    time::Duration,
};

use autd3_core::{
    autd3_device::NUM_TRANS_IN_UNIT,
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram,
};

enum Either {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

pub struct Simulator {
    addr: Either,
    port: u16,
    socket: Option<TcpStream>,
    tx_buf: Vec<u8>,
    rx_buf: Vec<u8>,
    timeout: Duration,
}

impl Simulator {
    pub fn new(port: u16) -> Self {
        Self {
            addr: Either::V4(Ipv4Addr::LOCALHOST),
            port,
            socket: None,
            tx_buf: Vec::new(),
            rx_buf: Vec::new(),
            timeout: Duration::from_millis(200),
        }
    }

    pub fn with_server_ip(self, ipv4: Ipv4Addr) -> Self {
        self.with_server_ipv4(ipv4)
    }

    pub fn with_server_ipv4(self, ipv4: Ipv4Addr) -> Self {
        Self {
            addr: Either::V4(ipv4),
            ..self
        }
    }

    pub fn with_server_ipv6(self, ipv6: Ipv6Addr) -> Self {
        Self {
            addr: Either::V6(ipv6),
            ..self
        }
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }
}

impl<T: Transducer> Link<T> for Simulator {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        let sockect_addr = match self.addr {
            Either::V4(ip) => SocketAddr::V4(SocketAddrV4::new(ip, self.port)),
            Either::V6(ip) => SocketAddr::V6(SocketAddrV6::new(ip, self.port, 0, 0)),
        };

        self.socket = Some(match TcpStream::connect(&sockect_addr) {
            Ok(s) => s,
            Err(e) => {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to connect to {:?}: {}",
                    sockect_addr, e
                )))
            }
        });

        let geometry_size = 1
            + std::mem::size_of::<u32>()
            + geometry.num_devices() * std::mem::size_of::<f32>() * 7;

        let mut geometry_buf = vec![0x00u8; geometry_size];
        unsafe {
            let mut cursor: *mut u8 = geometry_buf.as_mut_ptr();
            std::ptr::write(cursor, 3);
            cursor = cursor.add(1);

            std::ptr::write(cursor as *mut u32, geometry.num_devices() as u32);
            cursor = cursor.add(std::mem::size_of::<u32>());

            if geometry
                .device_map()
                .iter()
                .any(|&d| d != NUM_TRANS_IN_UNIT)
            {
                return Err(AUTDInternalError::LinkError(
                    "Simulator does not support non-AUTD3 device".to_string(),
                ));
            }

            (0..geometry.num_devices()).for_each(|dev| {
                let mut p = cursor as *mut f32;
                let tr = &geometry[dev * NUM_TRANS_IN_UNIT];
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
                cursor = cursor.add(std::mem::size_of::<f32>() * 7);
            });
        }

        if let Err(e) = self.socket.as_mut().unwrap().write(&geometry_buf) {
            return Err(AUTDInternalError::LinkError(format!(
                "Failed to send geometry data: {}",
                e
            )));
        }

        if !(0..50).any(|_| {
            std::thread::sleep(Duration::from_millis(100));

            let send_tmp = [2u8; 1];
            if self.socket.as_mut().unwrap().write(&send_tmp).is_err() {
                return false;
            }

            let mut buf = [0u8; 1];
            if self.socket.as_mut().unwrap().read(&mut buf).is_err() {
                return false;
            }

            buf[0] == 3
        }) {
            return Err(AUTDInternalError::LinkError(
                "Failed to initialize simulator".to_string(),
            ));
        }

        let tx_buf_size = 1
            + std::mem::size_of::<autd3_core::GlobalHeader>()
            + std::mem::size_of::<u16>() * geometry.num_transducers();
        let rx_buf_size = std::mem::size_of::<autd3_core::RxMessage>() * geometry.num_devices();

        self.tx_buf = vec![0; tx_buf_size];
        self.rx_buf = vec![0; rx_buf_size];

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if let Some(socket) = self.socket.take() {
            if let Err(e) = socket.shutdown(Shutdown::Both) {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to close socket: {}",
                    e
                )));
            }
        }

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if let Some(socket) = &mut self.socket {
            self.tx_buf[0] = 1;

            unsafe {
                std::ptr::copy_nonoverlapping(
                    tx.data().as_ptr() as *const u8,
                    self.tx_buf.as_mut_ptr().add(1),
                    tx.transmitting_size(),
                );
            }

            if let Err(e) = socket.write(&self.tx_buf[0..1 + tx.transmitting_size()]) {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to send data: {}",
                    e
                )));
            }
        }

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if let Some(socket) = &mut self.socket {
            if let Err(e) = socket.write(&[0x02u8]) {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to receive data: {}",
                    e
                )));
            }
            if let Err(e) = socket.read(&mut self.rx_buf) {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to receive data: {}",
                    e
                )));
            };

            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.rx_buf.as_ptr(),
                    rx.as_mut_ptr() as *mut u8,
                    self.rx_buf.len(),
                );
            }

            return Ok(true);
        }

        Err(AUTDInternalError::LinkClosed)
    }

    fn is_open(&self) -> bool {
        self.socket.is_some()
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
