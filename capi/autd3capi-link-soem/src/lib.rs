/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr},
    sync::{Arc, Mutex},
    time::Duration,
};

use autd3capi_def::{common::*, take_link, Level, LinkPtr, TimerStrategy};

use autd3_link_soem::{local::SOEM, remote::RemoteSOEM, EthernetAdapters};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetAdapterPointer() -> ConstPtr {
    Box::into_raw(Box::new(EthernetAdapters::new())) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGetAdapterSize(adapters: ConstPtr) -> u32 {
    cast!(adapters, EthernetAdapters).len() as u32
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetAdapter(
    adapters: ConstPtr,
    idx: u32,
    desc: *mut c_char,
    name: *mut c_char,
) {
    let adapter = &cast!(adapters, EthernetAdapters)[idx as usize];

    let name_ = std::ffi::CString::new(adapter.name().to_string()).unwrap();
    libc::strcpy(name, name_.as_ptr());
    let desc_ = std::ffi::CString::new(adapter.desc().to_string()).unwrap();
    libc::strcpy(desc, desc_.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFreeAdapterPointer(adapters: ConstPtr) {
    let _ = Box::from_raw(adapters as *mut EthernetAdapters);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEM() -> LinkPtr {
    LinkPtr::new(SOEM::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMSendCycle(soem: LinkPtr, cycle: u16) -> LinkPtr {
    LinkPtr::new(take_link!(soem, SOEM).with_send_cycle(cycle))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMSync0Cycle(soem: LinkPtr, cycle: u16) -> LinkPtr {
    LinkPtr::new(take_link!(soem, SOEM).with_sync0_cycle(cycle))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMBufSize(soem: LinkPtr, buf_size: u32) -> LinkPtr {
    LinkPtr::new(take_link!(soem, SOEM).with_buf_size(buf_size as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMTimerStrategy(
    soem: LinkPtr,
    timer_strategy: TimerStrategy,
) -> LinkPtr {
    LinkPtr::new(take_link!(soem, SOEM).with_timer_strategy(timer_strategy.into()))
}

#[repr(u8)]
pub enum SyncMode {
    FreeRun = 0,
    DC = 1,
}

impl From<SyncMode> for autd3_link_soem::SyncMode {
    fn from(mode: SyncMode) -> Self {
        match mode {
            SyncMode::FreeRun => autd3_link_soem::SyncMode::FreeRun,
            SyncMode::DC => autd3_link_soem::SyncMode::DC,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMSyncMode(soem: LinkPtr, mode: SyncMode) -> LinkPtr {
    LinkPtr::new(take_link!(soem, SOEM).with_sync_mode(mode.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMIfname(soem: LinkPtr, ifname: *const c_char) -> LinkPtr {
    LinkPtr::new(take_link!(soem, SOEM).with_ifname(CStr::from_ptr(ifname).to_str().unwrap()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMStateCheckInterval(
    soem: LinkPtr,
    interval_ms: u32,
) -> LinkPtr {
    LinkPtr::new(
        take_link!(soem, SOEM).with_state_check_interval(Duration::from_millis(interval_ms as _)),
    )
}

struct SOEMCallbackPtr(ConstPtr);
unsafe impl Send for SOEMCallbackPtr {}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMOnLost(soem: LinkPtr, on_lost_func: ConstPtr) -> LinkPtr {
    if on_lost_func.is_null() {
        return soem;
    }

    let out_f = Arc::new(Mutex::new(SOEMCallbackPtr(on_lost_func)));
    let out_func = move |msg: &str| {
        let msg = std::ffi::CString::new(msg).unwrap();
        let out_f =
            std::mem::transmute::<_, unsafe extern "C" fn(*const c_char)>(out_f.lock().unwrap().0);
        out_f(msg.as_ptr());
    };
    LinkPtr::new(take_link!(soem, SOEM).with_on_lost(out_func))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMLogLevel(soem: LinkPtr, level: Level) -> LinkPtr {
    LinkPtr::new(take_link!(soem, SOEM).with_log_level(level.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMLogFunc(
    soem: LinkPtr,
    out_func: ConstPtr,
    flush_func: ConstPtr,
) -> LinkPtr {
    if out_func.is_null() || flush_func.is_null() {
        return soem;
    }

    let out_f = Arc::new(Mutex::new(SOEMCallbackPtr(out_func)));
    let out_func = move |msg: &str| -> spdlog::Result<()> {
        let msg = std::ffi::CString::new(msg).unwrap();
        let out_f =
            std::mem::transmute::<_, unsafe extern "C" fn(*const c_char)>(out_f.lock().unwrap().0);
        out_f(msg.as_ptr());
        Ok(())
    };
    let flush_f = Arc::new(Mutex::new(SOEMCallbackPtr(flush_func)));
    let flush_func = move || -> spdlog::Result<()> {
        let flush_f = std::mem::transmute::<_, unsafe extern "C" fn()>(flush_f.lock().unwrap().0);
        flush_f();
        Ok(())
    };

    LinkPtr::new(
        take_link!(soem, SOEM).with_logger(get_logger_with_custom_func(out_func, flush_func)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkSOEMTimeout(soem: LinkPtr, timeout_ns: u64) -> LinkPtr {
    LinkPtr::new(take_link!(soem, SOEM).with_timeout(Duration::from_nanos(timeout_ns)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteSOEM(addr: *const c_char, err: *mut c_char) -> LinkPtr {
    LinkPtr::new(try_or_return!(
        RemoteSOEM::new(try_or_return!(
            try_or_return!(CStr::from_ptr(addr).to_str(), err, LinkPtr(NULL)).parse(),
            err,
            LinkPtr(NULL)
        )),
        err,
        LinkPtr(NULL)
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteSOEMTimeout(soem: LinkPtr, timeout_ns: u64) -> LinkPtr {
    LinkPtr::new(take_link!(soem, RemoteSOEM).with_timeout(Duration::from_nanos(timeout_ns)))
}
