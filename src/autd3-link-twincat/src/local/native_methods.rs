/*
 * File: native_methods.rs
 * Project: src
 * Created Date: 27/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use libc::c_void;
use libloading as lib;
use once_cell::sync::Lazy;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AmsNetId {
    pub b: [u8; 6],
}

#[repr(C)]
pub struct AmsAddr {
    pub net_id: AmsNetId,
    pub port: u16,
}

pub struct TcAds {
    pub tc_ads_port_open: lib::Symbol<'static, unsafe extern "C" fn() -> i32>,
    pub tc_ads_port_close: lib::Symbol<'static, unsafe extern "C" fn(i32) -> i32>,
    pub tc_ads_get_local_address:
        lib::Symbol<'static, unsafe extern "C" fn(i32, *mut AmsAddr) -> i32>,
    pub tc_ads_sync_write_req: lib::Symbol<
        'static,
        unsafe extern "C" fn(i32, *const AmsAddr, u32, u32, u32, *const c_void) -> i32,
    >,
    pub tc_ads_sync_read_req: lib::Symbol<
        'static,
        unsafe extern "C" fn(i32, *const AmsAddr, u32, u32, u32, *mut c_void, *mut u32) -> i32,
    >,
}

impl TcAds {
    fn new() -> TcAds {
        unsafe {
            TcAds {
                tc_ads_port_open: DLL.get(b"AdsPortOpenEx").unwrap(),
                tc_ads_port_close: DLL.get(b"AdsPortCloseEx").unwrap(),
                tc_ads_get_local_address: DLL.get(b"AdsGetLocalAddressEx").unwrap(),
                tc_ads_sync_write_req: DLL.get(b"AdsSyncWriteReqEx").unwrap(),
                tc_ads_sync_read_req: DLL.get(b"AdsSyncReadReqEx2").unwrap(),
            }
        }
    }
}

static DLL: Lazy<lib::Library> = Lazy::new(|| unsafe { lib::Library::new("TcAdsDll").unwrap() });
pub static TC_ADS: Lazy<TcAds> = Lazy::new(TcAds::new);
