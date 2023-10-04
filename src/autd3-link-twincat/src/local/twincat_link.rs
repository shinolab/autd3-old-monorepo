/*
 * File: twincat_link.rs
 * Project: src
 * Created Date: 27/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use libloading as lib;

use std::{ffi::c_void, time::Duration};

use lib::Library;

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    link::Link,
};

#[repr(C)]
#[derive(Copy, Clone)]
struct AmsNetId {
    pub b: [u8; 6],
}

#[repr(C)]
struct AmsAddr {
    pub net_id: AmsNetId,
    pub port: u16,
}

use crate::error::AdsError;

const INDEX_GROUP: u32 = 0x0304_0030;
const INDEX_OFFSET_BASE: u32 = 0x8100_0000;
const INDEX_OFFSET_BASE_READ: u32 = 0x8000_0000;
const PORT: u16 = 301;

/// Link using TwinCAT3
pub struct TwinCAT {
    port: i32,
    send_addr: AmsAddr,
    timeout: Duration,
    dll: Library,
}

impl TwinCAT {
    pub fn new() -> Result<Self, AUTDInternalError> {
        let dll = match unsafe { lib::Library::new("TcAdsDll") } {
            Ok(dll) => dll,
            Err(_) => {
                return Err(AUTDInternalError::LinkError(
                    "TcAdsDll not found. Please install TwinCAT3".to_owned(),
                ))
            }
        };

        Ok(Self {
            port: 0,
            send_addr: AmsAddr {
                net_id: AmsNetId {
                    b: [0, 0, 0, 0, 0, 0],
                },
                port: PORT,
            },
            timeout: Duration::ZERO,
            dll,
        })
    }

    /// Set timeout
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }
}

impl TwinCAT {
    fn port_open(&self) -> lib::Symbol<unsafe extern "C" fn() -> i32> {
        unsafe { self.dll.get(b"AdsPortOpenEx").unwrap() }
    }

    fn port_close(&self) -> lib::Symbol<unsafe extern "C" fn(i32) -> i32> {
        unsafe { self.dll.get(b"AdsPortCloseEx").unwrap() }
    }

    fn get_local_address(&self) -> lib::Symbol<unsafe extern "C" fn(i32, *mut AmsAddr) -> i32> {
        unsafe { self.dll.get(b"AdsGetLocalAddressEx").unwrap() }
    }

    fn sync_write_req(
        &self,
    ) -> lib::Symbol<unsafe extern "C" fn(i32, *const AmsAddr, u32, u32, u32, *const c_void) -> i32>
    {
        unsafe { self.dll.get(b"AdsSyncWriteReqEx").unwrap() }
    }

    fn sync_read_req(
        &self,
    ) -> lib::Symbol<
        unsafe extern "C" fn(i32, *const AmsAddr, u32, u32, u32, *mut c_void, *mut u32) -> i32,
    > {
        unsafe { self.dll.get(b"AdsSyncReadReqEx2").unwrap() }
    }
}

impl<T: Transducer> Link<T> for TwinCAT {
    fn open(&mut self, _devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
        unsafe {
            let port = self.port_open()();
            if port == 0 {
                return Err(AdsError::OpenPort.into());
            }
            self.port = port;

            let mut ams_addr: AmsAddr = std::mem::zeroed();
            let n_err = self.get_local_address()(port, &mut ams_addr as *mut _);
            if n_err != 0 {
                return Err(AdsError::GetLocalAddress(n_err).into());
            }
            self.send_addr.net_id = ams_addr.net_id;
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        unsafe {
            self.port_close()(self.port);
        }
        self.port = 0;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        unsafe {
            let n_err = self.sync_write_req()(
                self.port,
                &self.send_addr as *const _,
                INDEX_GROUP,
                INDEX_OFFSET_BASE,
                tx.all_data().len() as u32,
                tx.all_data().as_ptr() as *const c_void,
            );

            if n_err > 0 {
                Err(AdsError::SendData(n_err).into())
            } else {
                Ok(true)
            }
        }
    }

    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        let mut read_bytes: u32 = 0;
        unsafe {
            let n_err = self.sync_read_req()(
                self.port,
                &self.send_addr as *const _,
                INDEX_GROUP,
                INDEX_OFFSET_BASE_READ,
                std::mem::size_of_val(rx) as _,
                rx.as_mut_ptr() as *mut c_void,
                &mut read_bytes as *mut u32,
            );

            if n_err > 0 {
                Err(AdsError::ReadData(n_err).into())
            } else {
                Ok(true)
            }
        }
    }

    fn is_open(&self) -> bool {
        self.port > 0
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
