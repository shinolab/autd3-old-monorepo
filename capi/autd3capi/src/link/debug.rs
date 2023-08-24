/*
 * File: debug.rs
 * Project: link
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use crate::CallbackPtr;
use autd3capi_def::{common::*, take_link, Level, LinkPtr};
use std::{
    ffi::c_char,
    sync::{Arc, Mutex},
    time::Duration,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebug() -> LinkPtr {
    LinkPtr::new(Debug::new())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugWithLogLevel(debug: LinkPtr, level: Level) -> LinkPtr {
    LinkPtr::new(take_link!(debug, Debug).with_log_level(level.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugWithLogFunc(
    debug: LinkPtr,
    out_func: ConstPtr,
    flush_func: ConstPtr,
) -> LinkPtr {
    if out_func.is_null() || flush_func.is_null() {
        return debug;
    }

    let out_f = Arc::new(Mutex::new(CallbackPtr(out_func)));
    let out_func = move |msg: &str| -> spdlog::Result<()> {
        let msg = std::ffi::CString::new(msg).unwrap();
        let out_f =
            std::mem::transmute::<_, unsafe extern "C" fn(*const c_char)>(out_f.lock().unwrap().0);
        out_f(msg.as_ptr());
        Ok(())
    };
    let flush_f = Arc::new(Mutex::new(CallbackPtr(flush_func)));
    let flush_func = move || -> spdlog::Result<()> {
        let flush_f = std::mem::transmute::<_, unsafe extern "C" fn()>(flush_f.lock().unwrap().0);
        flush_f();
        Ok(())
    };

    LinkPtr::new(
        take_link!(debug, Debug).with_logger(get_logger_with_custom_func(out_func, flush_func)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkDebugWithTimeout(debug: LinkPtr, timeout_ns: u64) -> LinkPtr {
    LinkPtr::new(take_link!(debug, Debug).with_timeout(Duration::from_nanos(timeout_ns)))
}
