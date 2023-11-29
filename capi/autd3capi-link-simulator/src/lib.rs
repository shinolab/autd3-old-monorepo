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
    net::Ipv4Addr,
    time::Duration,
};

use autd3capi_def::*;

use autd3_link_simulator::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LinkSimulatorBuilderPtr(pub ConstPtr);

impl LinkSimulatorBuilderPtr {
    pub fn new(builder: SimulatorBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultLinkSimulatorBuilder {
    pub result: LinkSimulatorBuilderPtr,
    pub err_len: u32,
    pub err: ConstPtr,
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
) -> ResultLinkSimulatorBuilder {
    let addr = match CStr::from_ptr(addr).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultLinkSimulatorBuilder {
                result: LinkSimulatorBuilderPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    let addr = match addr.parse::<Ipv4Addr>() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultLinkSimulatorBuilder {
                result: LinkSimulatorBuilderPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    ResultLinkSimulatorBuilder {
        result: LinkSimulatorBuilderPtr::new(
            Box::from_raw(simulator.0 as *mut SimulatorBuilder).with_server_ip(addr),
        ),
        err_len: 0,
        err: std::ptr::null_mut(),
    }
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
pub unsafe extern "C" fn AUTDLinkSimulatorIntoBuilder(
    simulator: LinkSimulatorBuilderPtr,
) -> LinkBuilderPtr {
    LinkBuilderPtr::new(Box::from_raw(simulator.0 as *mut SimulatorBuilder).blocking())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSimulatorUpdateGeometry(
    simulator: LinkPtr,
    geometry: GeometryPtr,
) -> ResultI32 {
    cast_mut!(simulator.0, Box<SimulatorSync>)
        .update_geometry(cast!(geometry.0, Geometry))
        .into()
}
