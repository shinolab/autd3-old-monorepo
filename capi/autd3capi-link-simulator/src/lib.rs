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

use autd3_link_simulator::Simulator;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulator(port: u16) -> LinkPtr {
    LinkPtr::new(Simulator::new(port))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorAddr(
    simulator: LinkPtr,
    addr: *const c_char,
    err: *mut c_char,
) -> LinkPtr {
    LinkPtr::new(try_or_return!(
        take_link!(simulator, Simulator).with_server_ip(try_or_return!(
            CStr::from_ptr(addr).to_str(),
            err,
            LinkPtr(NULL)
        )),
        err,
        LinkPtr(NULL)
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorTimeout(simulator: LinkPtr, timeout_ns: u64) -> LinkPtr {
    LinkPtr::new(take_link!(simulator, Simulator).with_timeout(Duration::from_nanos(timeout_ns)))
}
