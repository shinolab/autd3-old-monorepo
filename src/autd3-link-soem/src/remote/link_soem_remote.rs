/*
 * File: link_soem_remote.rs
 * Project: src
 * Created Date: 21/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    io::{Read, Write},
    net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram,
};

pub struct RemoteSOEM {
    server_addrs: Vec<SocketAddr>,
    socket: Option<TcpStream>,
    tx_buf: Vec<u8>,
    rx_buf: Vec<u8>,
    timeout: Duration,
}

impl RemoteSOEM {
    pub fn new<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        Ok(Self {
            server_addrs: addr.to_socket_addrs()?.collect(),
            socket: None,
            tx_buf: vec![],
            rx_buf: vec![],
            timeout: Duration::from_millis(20),
        })
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }
}

impl Link for RemoteSOEM {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        self.socket = Some(match TcpStream::connect(&self.server_addrs[..]) {
            Ok(s) => s,
            Err(e) => {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to connect to {:?}: {}",
                    self.server_addrs, e
                )))
            }
        });

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
                    rx.messages_mut().as_mut_ptr() as *mut u8,
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
