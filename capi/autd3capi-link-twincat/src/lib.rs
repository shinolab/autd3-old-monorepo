/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
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

use autd3capi_def::{common::*, LinkBuilderPtr};

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
pub struct LinkRemoteTwinCATBuilderPtr(pub ConstPtr);

impl LinkRemoteTwinCATBuilderPtr {
    pub fn new(builder: RemoteTwinCATBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCAT(
    server_ams_net_id: *const c_char,
    err: *mut c_char,
) -> LinkRemoteTwinCATBuilderPtr {
    LinkRemoteTwinCATBuilderPtr::new(RemoteTwinCAT::builder(try_or_return!(
        CStr::from_ptr(server_ams_net_id).to_str(),
        err,
        LinkRemoteTwinCATBuilderPtr(NULL)
    )))
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
pub unsafe extern "C" fn AUTDLinkAuditIntoBuilder(
    twincat: LinkRemoteTwinCATBuilderPtr,
) -> LinkBuilderPtr {
    LinkBuilderPtr::new(*Box::from_raw(twincat.0 as *mut RemoteTwinCATBuilder))
}
