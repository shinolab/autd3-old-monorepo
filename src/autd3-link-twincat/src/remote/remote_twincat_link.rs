/*
 * File: twincat_link.rs
 * Project: src
 * Created Date: 27/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::{ffi::CString, time::Duration};

use itertools::Itertools;
use libc::c_long;

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram,
};

use crate::{error::AdsError, remote::native_methods::*};

const INDEX_GROUP: u32 = 0x0304_0030;
const INDEX_OFFSET_BASE: u32 = 0x8100_0000;
const INDEX_OFFSET_BASE_READ: u32 = 0x8000_0000;
const PORT: u16 = 301;

pub struct RemoteTwinCAT {
    port: c_long,
    server_ams_net_id: String,
    server_ip: Option<String>,
    client_ams_net_id: Option<String>,
    net_id: AmsNetId,
    timeout: Duration,
}

impl RemoteTwinCAT {
    pub fn new<S: Into<String>>(server_ams_net_id: S) -> Result<Self, AdsError> {
        Ok(Self {
            port: 0,
            server_ams_net_id: server_ams_net_id.into(),
            server_ip: None,
            client_ams_net_id: None,
            net_id: AmsNetId { b: [0; 6] },
            timeout: Duration::from_millis(200),
        })
    }

    pub fn with_server_ip<S: Into<String>>(mut self, server_ip: S) -> Self {
        self.server_ip = Some(server_ip.into());
        self
    }

    pub fn with_client_ams_net_id<S: Into<String>>(mut self, client_ams_net_id: S) -> Self {
        self.client_ams_net_id = Some(client_ams_net_id.into());
        self
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }
}

impl<T: Transducer> Link<T> for RemoteTwinCAT {
    fn open(&mut self, _geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        let octets = self
            .server_ams_net_id
            .split('.')
            .map(|octet| octet.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        if octets.len() != 6 {
            return Err(AdsError::AmsNetIdParse.into());
        }

        let ip = if let Some(server_ip) = self.server_ip.take() {
            server_ip
        } else {
            octets[0..4].iter().map(|v| v.to_string()).join(".")
        };

        if let Some(client_ams_net_id) = self.client_ams_net_id.take() {
            let local_octets = client_ams_net_id
                .split('.')
                .map(|octet| octet.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            if local_octets.len() != 6 {
                return Err(AdsError::AmsNetIdParse.into());
            }

            let local_addr = AmsNetId {
                b: [
                    local_octets[0],
                    local_octets[1],
                    local_octets[2],
                    local_octets[3],
                    local_octets[4],
                    local_octets[5],
                ],
            };
            unsafe {
                AdsCSetLocalAddress(local_addr);
            }
        }

        self.net_id = AmsNetId {
            b: [
                octets[0], octets[1], octets[2], octets[3], octets[4], octets[5],
            ],
        };

        let ip = CString::new(ip).unwrap();
        let res = unsafe { AdsCAddRoute(self.net_id, ip.as_c_str().as_ptr()) };
        if res != 0 {
            return Err(AdsError::AmsAddRoute(res as _).into());
        }

        self.port = unsafe { AdsCPortOpenEx() };

        if self.port == 0 {
            return Err(AdsError::OpenPort.into());
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if self.port == 0 {
            return Ok(());
        }

        unsafe {
            if AdsCPortCloseEx(self.port) != 0 {
                return Err(AdsError::ClosePort.into());
            }
        }

        self.port = 0;

        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        let addr = AmsAddr {
            net_id: self.net_id,
            port: PORT,
        };

        let res = unsafe {
            AdsCSyncWriteReqEx(
                self.port,
                &addr as _,
                INDEX_GROUP,
                INDEX_OFFSET_BASE,
                tx.transmitting_size() as _,
                tx.data().as_ptr() as _,
            )
        };

        if res == 0 {
            return Ok(true);
        }

        if res == ADSERR_DEVICE_INVALIDSIZE {
            return Err(AdsError::DeviceInvalidSize.into());
        }

        Err(AdsError::SendData(res as _).into())
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        let addr = AmsAddr {
            net_id: self.net_id,
            port: PORT,
        };

        let mut receive_bytes: u32 = 0;
        let res = unsafe {
            AdsCSyncReadReqEx2(
                self.port,
                &addr as _,
                INDEX_GROUP,
                INDEX_OFFSET_BASE_READ,
                std::mem::size_of_val(rx.messages()) as _,
                rx.messages_mut().as_mut_ptr() as _,
                &mut receive_bytes as _,
            )
        };

        if res == 0 {
            return Ok(true);
        }

        Err(AdsError::ReadData(res as _).into())
    }

    fn is_open(&self) -> bool {
        self.port > 0
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
