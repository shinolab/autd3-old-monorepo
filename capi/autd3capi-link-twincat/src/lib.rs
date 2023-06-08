/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2023
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

use autd3capi_def::{common::*, take_link, LinkPtr};

use autd3_link_twincat::{local::TwinCAT, remote::RemoteTwinCAT};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCAT(err: *mut c_char) -> LinkPtr {
    LinkPtr::new(try_or_return!(TwinCAT::new(), err, LinkPtr(NULL)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkTwinCATTimeout(twincat: LinkPtr, timeout_ns: u64) -> LinkPtr {
    LinkPtr::new(take_link!(twincat, TwinCAT).with_timeout(Duration::from_nanos(timeout_ns)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCAT(
    server_ams_net_id: *const c_char,
    err: *mut c_char,
) -> LinkPtr {
    LinkPtr::new(try_or_return!(
        RemoteTwinCAT::new(try_or_return!(
            CStr::from_ptr(server_ams_net_id).to_str(),
            err,
            LinkPtr(NULL)
        )),
        err,
        LinkPtr(NULL)
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATServerIP(
    twincat: LinkPtr,
    addr: *const c_char,
) -> LinkPtr {
    LinkPtr::new(
        take_link!(twincat, RemoteTwinCAT).with_server_ip(CStr::from_ptr(addr).to_str().unwrap()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATClientAmsNetId(
    twincat: LinkPtr,
    id: *const c_char,
) -> LinkPtr {
    LinkPtr::new(
        take_link!(twincat, RemoteTwinCAT)
            .with_client_ams_net_id(CStr::from_ptr(id).to_str().unwrap()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATTimeout(
    twincat: LinkPtr,
    timeout_ns: u64,
) -> LinkPtr {
    LinkPtr::new(take_link!(twincat, RemoteTwinCAT).with_timeout(Duration::from_nanos(timeout_ns)))
}
