#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr},
    sync::{Arc, Mutex},
    time::Duration,
};

use autd3capi::Level;
use autd3capi_common::*;

use autd3_link_soem::{
    local::{SOEMBuilder, SOEM},
    remote::{Filled, RemoteSOEM, RemoteSOEMBuilder},
    EthernetAdapters,
};

#[no_mangle]
pub unsafe extern "C" fn AUTDGetAdapterPointer(len: *mut u32) -> ConstPtr {
    let adapters = EthernetAdapters::new();
    unsafe {
        *len = adapters.len() as u32;
    }
    Box::into_raw(Box::new(adapters)) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGetAdapter(
    adapters: ConstPtr,
    idx: u32,
    desc: *mut c_char,
    name: *mut c_char,
) {
    let adapter = &cast_without_ownership!(adapters, EthernetAdapters)[idx as usize];

    let name_ = std::ffi::CString::new(adapter.name().to_string()).unwrap();
    libc::strcpy(name, name_.as_ptr());
    let desc_ = std::ffi::CString::new(adapter.desc().to_string()).unwrap();
    libc::strcpy(desc, desc_.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFreeAdapterPointer(adapters: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(adapters as *mut EthernetAdapters);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEM() -> ConstPtr {
    Box::into_raw(Box::new(SOEM::builder())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMSendCycle(builder: ConstPtr, cycle: u16) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder).send_cycle(cycle),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMSync0Cycle(builder: ConstPtr, cycle: u16) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder).sync0_cycle(cycle),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMBufSize(builder: ConstPtr, buf_size: u32) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder).buf_size(buf_size as _),
        )) as _
    }
}

#[repr(u8)]
pub enum TimerStrategy {
    Sleep = 0,
    NativeTimer = 1,
    BusyWait = 2,
}

impl From<TimerStrategy> for autd3::prelude::TimerStrategy {
    fn from(strategy: TimerStrategy) -> Self {
        match strategy {
            TimerStrategy::Sleep => autd3::prelude::TimerStrategy::Sleep,
            TimerStrategy::NativeTimer => autd3::prelude::TimerStrategy::NativeTimer,
            TimerStrategy::BusyWait => autd3::prelude::TimerStrategy::BusyWait,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMTimerStrategy(
    builder: ConstPtr,
    timer_strategy: TimerStrategy,
) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder).timer_strategy(timer_strategy.into()),
        )) as _
    }
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
pub unsafe extern "C" fn AUTDLinkSOEMSyncMode(builder: ConstPtr, mode: SyncMode) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder).sync_mode(mode.into()),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMIfname(builder: ConstPtr, ifname: *const c_char) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder)
                .ifname(CStr::from_ptr(ifname).to_str().unwrap()),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMStateCheckInterval(
    builder: ConstPtr,
    interval_ms: u32,
) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder)
                .state_check_interval(Duration::from_millis(interval_ms as _)),
        )) as _
    }
}

struct Callback(ConstPtr);
unsafe impl Send for Callback {}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMOnLost(builder: ConstPtr, on_lost_func: ConstPtr) -> ConstPtr {
    unsafe {
        if on_lost_func.is_null() {
            return builder;
        }

        let out_f = Arc::new(Mutex::new(Callback(on_lost_func)));
        let out_func = move |msg: &str| {
            let msg = std::ffi::CString::new(msg).unwrap();
            let out_f = std::mem::transmute::<_, unsafe extern "C" fn(*const c_char)>(
                out_f.lock().unwrap().0,
            );
            out_f(msg.as_ptr());
        };
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder).on_lost(out_func),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMLogLevel(builder: ConstPtr, level: Level) -> ConstPtr {
    Box::into_raw(Box::new(
        Box::from_raw(builder as *mut SOEMBuilder).level(level.into()),
    )) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMLogFunc(
    builder: ConstPtr,
    level: Level,
    out_func: ConstPtr,
    flush_func: ConstPtr,
) -> ConstPtr {
    unsafe {
        if out_func.is_null() || flush_func.is_null() {
            return builder;
        }

        let out_f = Arc::new(Mutex::new(Callback(out_func)));
        let out_func = move |msg: &str| -> spdlog::Result<()> {
            let msg = std::ffi::CString::new(msg).unwrap();
            let out_f = std::mem::transmute::<_, unsafe extern "C" fn(*const c_char)>(
                out_f.lock().unwrap().0,
            );
            out_f(msg.as_ptr());
            Ok(())
        };
        let flush_f = Arc::new(Mutex::new(Callback(flush_func)));
        let flush_func = move || -> spdlog::Result<()> {
            let flush_f =
                std::mem::transmute::<_, unsafe extern "C" fn()>(flush_f.lock().unwrap().0);
            flush_f();
            Ok(())
        };

        let logger = get_logger_with_custom_func(level.into(), out_func, flush_func);

        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder).logger(logger),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMTimeout(builder: ConstPtr, timeout_ns: u64) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut SOEMBuilder).timeout(Duration::from_nanos(timeout_ns)),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkSOEMBuild(builder: ConstPtr) -> ConstPtr {
    unsafe {
        let builder = Box::from_raw(builder as *mut SOEMBuilder);
        let link: Box<Box<L>> = Box::new(Box::new(builder.build()));
        Box::into_raw(link) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkRemoteSOEM(addr: *const c_char, port: u16) -> ConstPtr {
    Box::into_raw(Box::new(
        RemoteSOEM::builder()
            .port(port)
            .addr(CStr::from_ptr(addr).to_str().unwrap()),
    )) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkRemoteSOEMTimeout(builder: ConstPtr, timeout_ns: u64) -> ConstPtr {
    unsafe {
        Box::into_raw(Box::new(
            Box::from_raw(builder as *mut RemoteSOEMBuilder<Filled, Filled>)
                .timeout(Duration::from_nanos(timeout_ns)),
        )) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkRemoteSOEMBuild(builder: ConstPtr) -> ConstPtr {
    unsafe {
        let builder = Box::from_raw(builder as *mut RemoteSOEMBuilder<Filled, Filled>);
        let link: Box<Box<L>> = Box::new(Box::new(builder.build()));
        Box::into_raw(link) as _
    }
}
