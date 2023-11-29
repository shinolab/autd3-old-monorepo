/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr},
    time::Duration,
};

use autd3capi_def::*;

use autd3_link_twincat::{local::twincat_link::*, remote::remote_twincat_link::*};

#[repr(C)]
pub struct LinkTwinCATBuilderPtr(pub ConstPtr);

impl LinkTwinCATBuilderPtr {
    pub fn new(builder: TwinCATBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCAT() -> LinkTwinCATBuilderPtr {
    LinkTwinCATBuilderPtr::new(TwinCAT::builder())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCATWithTimeout(
    twincat: LinkTwinCATBuilderPtr,
    timeout_ns: u64,
) -> LinkTwinCATBuilderPtr {
    LinkTwinCATBuilderPtr::new(
        Box::from_raw(twincat.0 as *mut TwinCATBuilder)
            .with_timeout(Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCATIntoBuilder(
    twincat: LinkTwinCATBuilderPtr,
) -> LinkBuilderPtr {
    LinkBuilderPtr::new(*Box::from_raw(twincat.0 as *mut TwinCATBuilder))
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LinkRemoteTwinCATBuilderPtr(pub ConstPtr);

impl LinkRemoteTwinCATBuilderPtr {
    pub fn new(builder: RemoteTwinCATBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultLinkRemoteTwinCATBuilder {
    pub result: LinkRemoteTwinCATBuilderPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCAT(
    server_ams_net_id: *const c_char,
) -> ResultLinkRemoteTwinCATBuilder {
    match CStr::from_ptr(server_ams_net_id).to_str() {
        Ok(v) => {
            let builder = RemoteTwinCAT::builder(v);
            ResultLinkRemoteTwinCATBuilder {
                result: LinkRemoteTwinCATBuilderPtr::new(builder),
                err_len: 0,
                err: std::ptr::null_mut(),
            }
        }
        Err(e) => {
            let err = e.to_string();
            ResultLinkRemoteTwinCATBuilder {
                result: LinkRemoteTwinCATBuilderPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATWithServerIP(
    twincat: LinkRemoteTwinCATBuilderPtr,
    addr: *const c_char,
) -> LinkRemoteTwinCATBuilderPtr {
    LinkRemoteTwinCATBuilderPtr::new(
        Box::from_raw(twincat.0 as *mut RemoteTwinCATBuilder)
            .with_server_ip(CStr::from_ptr(addr).to_str().unwrap()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATWithClientAmsNetId(
    twincat: LinkRemoteTwinCATBuilderPtr,
    id: *const c_char,
) -> LinkRemoteTwinCATBuilderPtr {
    LinkRemoteTwinCATBuilderPtr::new(
        Box::from_raw(twincat.0 as *mut RemoteTwinCATBuilder)
            .with_client_ams_net_id(CStr::from_ptr(id).to_str().unwrap()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATWithTimeout(
    twincat: LinkRemoteTwinCATBuilderPtr,
    timeout_ns: u64,
) -> LinkRemoteTwinCATBuilderPtr {
    LinkRemoteTwinCATBuilderPtr::new(
        Box::from_raw(twincat.0 as *mut RemoteTwinCATBuilder)
            .with_timeout(Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATIntoBuilder(
    twincat: LinkRemoteTwinCATBuilderPtr,
) -> LinkBuilderPtr {
    LinkBuilderPtr::new(*Box::from_raw(twincat.0 as *mut RemoteTwinCATBuilder))
}
