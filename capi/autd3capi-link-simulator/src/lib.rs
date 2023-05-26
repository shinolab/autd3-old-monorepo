#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr},
    time::Duration,
};

use autd3capi_common::*;

use autd3_link_simulator::{Filled, Simulator, SimulatorBuilder};

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulator(port: u16) -> ConstPtr {
    Box::into_raw(Box::new(Simulator::builder().port(port))) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorAddr(builder: ConstPtr, addr: *const c_char) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SimulatorBuilder<Filled>)
                .addr(CStr::from_ptr(addr).to_str().unwrap()),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorTimeout(builder: ConstPtr, timeout_ns: u64) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SimulatorBuilder<Filled>)
                .timeout(Duration::from_nanos(timeout_ns)),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSimulatorBuild(builder: ConstPtr) -> ConstPtr {
    unsafe {
        let builder = Box::from_raw(builder as *mut SimulatorBuilder<Filled>);
        let link: Box<Box<L>> = Box::new(Box::new(builder.build()));
        Box::into_raw(link) as _
    }
}
