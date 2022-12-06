/*
 * File: twincat_link.rs
 * Project: src
 * Created Date: 27/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;
use libc::c_void;

use autd3_core::{
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, RxMessage, TxDatagram,
};

use crate::{error::AdsError, native_methods::*};

const INDEX_GROUP: u32 = 0x0304_0030;
const INDEX_OFFSET_BASE: u32 = 0x8100_0000;
const INDEX_OFFSET_BASE_READ: u32 = 0x8000_0000;
const PORT: u16 = 301;

pub struct TwinCAT {
    port: i32,
    send_addr: AmsAddr,
}

impl TwinCAT {
    pub fn new() -> Self {
        unsafe {
            let ams_addr: AmsAddr = std::mem::zeroed();
            Self {
                port: 0,
                send_addr: AmsAddr {
                    net_id: ams_addr.net_id,
                    port: PORT,
                },
            }
        }
    }
}

impl Default for TwinCAT {
    fn default() -> Self {
        Self::new()
    }
}

impl Link for TwinCAT {
    fn open<T: Transducer>(&mut self, _geometry: &Geometry<T>) -> anyhow::Result<()> {
        unsafe {
            let port = (TC_ADS.tc_ads_port_open)();
            if port == 0 {
                return Err(AdsError::OpenPort.into());
            }
            self.port = port;

            let mut ams_addr: AmsAddr = std::mem::zeroed();
            let n_err = (TC_ADS.tc_ads_get_local_address)(port, &mut ams_addr as *mut _);
            if n_err != 0 {
                return Err(AdsError::GetLocalAddress(n_err).into());
            }
            self.send_addr.net_id = ams_addr.net_id;
        }

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        unsafe {
            (TC_ADS.tc_ads_port_close)(self.port);
        }
        self.port = 0;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool> {
        unsafe {
            let n_err = (TC_ADS.tc_ads_sync_write_req)(
                self.port,
                &self.send_addr as *const _,
                INDEX_GROUP,
                INDEX_OFFSET_BASE,
                tx.transmitting_size() as u32,
                tx.data().as_ptr() as *const c_void,
            );

            if n_err > 0 {
                Err(AdsError::SendData(n_err).into())
            } else {
                Ok(true)
            }
        }
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool> {
        let mut read_bytes: u32 = 0;
        unsafe {
            let n_err = (TC_ADS.tc_ads_sync_read_req)(
                self.port,
                &self.send_addr as *const _,
                INDEX_GROUP,
                INDEX_OFFSET_BASE_READ,
                (rx.messages().len() * std::mem::size_of::<RxMessage>()) as u32,
                rx.messages_mut().as_mut_ptr() as *mut c_void,
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
}
