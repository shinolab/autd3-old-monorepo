/*
 * File: link_soem_remote.rs
 * Project: src
 * Created Date: 21/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    io::{Read, Write},
    marker::PhantomData,
    net::{Shutdown, TcpStream},
    time::Duration,
};

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram,
};

pub struct RemoteSOEM {
    addr: String,
    port: u16,
    socket: Option<TcpStream>,
    tx_buf: Vec<u8>,
    rx_buf: Vec<u8>,
    timeout: Duration,
}

pub struct Empty;
pub struct Filled;

pub struct RemoteSOEMBuilder<Addr, Port> {
    addr: String,
    addr_: PhantomData<Addr>,
    port: u16,
    port_: PhantomData<Port>,
    timeout: Duration,
}

impl RemoteSOEM {
    fn new(addr: String, port: u16, timeout: Duration) -> Self {
        Self {
            addr,
            port,
            socket: None,
            tx_buf: vec![],
            rx_buf: vec![],
            timeout,
        }
    }

    pub fn builder() -> RemoteSOEMBuilder<Empty, Empty> {
        RemoteSOEMBuilder::new()
    }
}

impl RemoteSOEMBuilder<Empty, Empty> {
    fn new() -> Self {
        Self {
            addr: String::new(),
            addr_: PhantomData,
            port: 0,
            port_: PhantomData,
            timeout: Duration::from_millis(20),
        }
    }
}

impl RemoteSOEMBuilder<Filled, Filled> {
    pub fn build(self) -> RemoteSOEM {
        RemoteSOEM::new(self.addr, self.port, self.timeout)
    }
}

impl<Addr> RemoteSOEMBuilder<Addr, Empty> {
    pub fn port(mut self, port: u16) -> RemoteSOEMBuilder<Addr, Filled> {
        self.port = port;
        unsafe { std::mem::transmute(self) }
    }
}

impl<Port> RemoteSOEMBuilder<Empty, Port> {
    pub fn addr<S: Into<String>>(mut self, addr: S) -> RemoteSOEMBuilder<Filled, Port> {
        self.addr = addr.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Addr, Port> RemoteSOEMBuilder<Addr, Port> {
    pub fn timeout(mut self, timeout: Duration) -> RemoteSOEMBuilder<Addr, Port> {
        self.timeout = timeout;
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: Transducer> Link<T> for RemoteSOEM {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        let addr = format!("{}:{}", self.addr, self.port);
        self.socket = Some(match TcpStream::connect(&addr) {
            Ok(s) => s,
            Err(e) => {
                return Err(AUTDInternalError::LinkError(format!(
                    "Failed to connect to {}: {}",
                    addr, e
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

        return Err(AUTDInternalError::LinkClosed);
    }

    fn is_open(&self) -> bool {
        self.socket.is_some()
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
