#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr},
    time::Duration,
};

use autd3capi_common::*;

use autd3_link_twincat::{
    local::{TwinCAT, TwinCATBuilder},
    remote::{Filled, RemoteTwinCAT, RemoteTwinCATBuilder},
};

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTwinCAT() -> ConstPtr {
    Box::into_raw(Box::new(TwinCAT::builder())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTwinCATTimeout(builder: ConstPtr, timeout_ns: u64) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut TwinCATBuilder).timeout(Duration::from_nanos(timeout_ns)),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkTwinCATBuild(builder: ConstPtr, err: *mut c_char) -> ConstPtr {
    unsafe {
        let builder = Box::from_raw(builder as *mut TwinCATBuilder);
        let link = try_or_return!(builder.build(), err, NULL);
        let link: Box<Box<L>> = Box::new(Box::new(link));
        Box::into_raw(link) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCAT(server_ams_net_id: *const c_char) -> ConstPtr {
    Box::into_raw(Box::new(RemoteTwinCAT::builder().server_ams_net_id(
        CStr::from_ptr(server_ams_net_id).to_str().unwrap(),
    ))) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATServerIP(
    builder: ConstPtr,
    addr: *const c_char,
) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut RemoteTwinCATBuilder<Filled>)
                .server_ip_addr(CStr::from_ptr(addr).to_str().unwrap()),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATClientAmsNetId(
    builder: ConstPtr,
    id: *const c_char,
) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut RemoteTwinCATBuilder<Filled>)
                .client_ams_net_id(CStr::from_ptr(id).to_str().unwrap()),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATTimeout(
    builder: ConstPtr,
    timeout_ns: u64,
) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut RemoteTwinCATBuilder<Filled>)
                .timeout(Duration::from_nanos(timeout_ns)),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCATBuild(
    builder: ConstPtr,
    err: *mut c_char,
) -> ConstPtr {
    unsafe {
        let builder = Box::from_raw(builder as *mut RemoteTwinCATBuilder<Filled>);
        let link = try_or_return!(builder.build(), err, NULL);
        let link: Box<Box<L>> = Box::new(Box::new(link));
        Box::into_raw(link) as _
    }
}
