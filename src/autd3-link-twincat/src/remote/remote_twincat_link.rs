/*
 * File: twincat_link.rs
 * Project: src
 * Created Date: 27/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::{ffi::CString, marker::PhantomData, time::Duration};

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
    ip: String,
    net_id: AmsNetId,
    timeout: Duration,
}

pub struct Empty;
pub struct Filled;

pub struct RemoteTwinCATBuilder<ServerAmsNetId> {
    server_ip: String,
    server_ams_net_id: String,
    client_ams_net_id: String,
    server_ams_net_id_: PhantomData<ServerAmsNetId>,
    timeout: Duration,
}

impl RemoteTwinCAT {
    fn new(
        server_ip: &str,
        server_ams_net_id: &str,
        client_ams_net_id: &str,
        timeout: Duration,
    ) -> Result<Self, AdsError> {
        let octets = server_ams_net_id
            .split('.')
            .map(|octet| octet.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        if octets.len() != 6 {
            return Err(AdsError::AmsNetIdParse);
        }

        let ip = if server_ip.is_empty() {
            octets[0..4].iter().map(|v| v.to_string()).join(".")
        } else {
            server_ip.to_owned()
        };

        if !client_ams_net_id.is_empty() {
            let local_octets = client_ams_net_id
                .split('.')
                .map(|octet| octet.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            if local_octets.len() != 6 {
                return Err(AdsError::AmsNetIdParse);
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

        let net_id = AmsNetId {
            b: [
                octets[0], octets[1], octets[2], octets[3], octets[4], octets[5],
            ],
        };

        Ok(Self {
            port: 0,
            ip,
            net_id,
            timeout,
        })
    }

    pub fn builder() -> RemoteTwinCATBuilder<Empty> {
        RemoteTwinCATBuilder::new()
    }
}

impl RemoteTwinCATBuilder<Empty> {
    fn new() -> Self {
        Self {
            server_ip: String::new(),
            server_ams_net_id: String::new(),
            client_ams_net_id: String::new(),
            server_ams_net_id_: PhantomData,
            timeout: Duration::ZERO,
        }
    }

    pub fn server_ams_net_id(mut self, ams_net_id: &str) -> RemoteTwinCATBuilder<Filled> {
        self.server_ams_net_id = ams_net_id.to_owned();
        unsafe { std::mem::transmute(self) }
    }
}

impl<ServerAmsNetId> RemoteTwinCATBuilder<ServerAmsNetId> {
    pub fn client_ams_net_id(mut self, ams_net_id: &str) -> Self {
        self.client_ams_net_id = ams_net_id.to_owned();
        self
    }

    pub fn server_ip_addr(mut self, ipv4: &str) -> Self {
        self.server_ip = ipv4.to_owned();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl RemoteTwinCATBuilder<Filled> {
    pub fn build(self) -> Result<RemoteTwinCAT, AdsError> {
        RemoteTwinCAT::new(
            &self.server_ip,
            &self.server_ams_net_id,
            &self.client_ams_net_id,
            self.timeout,
        )
    }
}

impl<T: Transducer> Link<T> for RemoteTwinCAT {
    fn open(&mut self, _geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        let ip = CString::new(self.ip.to_owned()).unwrap();
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
                (rx.messages().len() * std::mem::size_of::<autd3_core::RxMessage>()) as _,
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
