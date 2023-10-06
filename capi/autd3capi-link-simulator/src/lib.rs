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

use autd3_link_simulator::*;

#[repr(C)]
pub struct LinkSimulatorBuilderPtr(pub ConstPtr);

impl LinkSimulatorBuilderPtr {
    pub fn new(builder: SimulatorBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulator(port: u16) -> LinkSimulatorBuilderPtr {
    LinkSimulatorBuilderPtr::new(Simulator::builder(port))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorWithAddr(
    simulator: LinkSimulatorBuilderPtr,
    addr: *const c_char,
    err: *mut c_char,
) -> LinkSimulatorBuilderPtr {
    LinkSimulatorBuilderPtr::new(
        Box::from_raw(simulator.0 as *mut SimulatorBuilder).with_server_ip(try_or_return!(
            try_or_return!(
                CStr::from_ptr(addr).to_str(),
                err,
                LinkSimulatorBuilderPtr(NULL)
            )
            .parse(),
            err,
            LinkSimulatorBuilderPtr(NULL)
        )),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorWithTimeout(
    simulator: LinkSimulatorBuilderPtr,
    timeout_ns: u64,
) -> LinkSimulatorBuilderPtr {
    LinkSimulatorBuilderPtr::new(
        Box::from_raw(simulator.0 as *mut SimulatorBuilder)
            .with_timeout(Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditIntoBuilder(
    simulator: LinkSimulatorBuilderPtr,
) -> LinkBuilderPtr {
    LinkBuilderPtr::new(*Box::from_raw(simulator.0 as *mut SimulatorBuilder))
}
