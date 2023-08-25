/*
 * File: log.rs
 * Project: link
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use crate::CallbackPtr;
use autd3capi_def::{
    common::{
        autd3::link::{log::LogImpl, Log},
        *,
    },
    take_link, Level, LinkPtr,
};
use std::{
    ffi::c_char,
    sync::{Arc, Mutex},
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkLog(link: LinkPtr) -> LinkPtr {
    let link: Box<Box<L>> = Box::from_raw(link.0 as *mut Box<L>);
    LinkPtr::new(link.with_log())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkLogWithLogLevel(log: LinkPtr, level: Level) -> LinkPtr {
    LinkPtr::new(take_link!(log, LogImpl<DynamicTransducer, Box<L>>).with_log_level(level.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkLogWithLogFunc(
    log: LinkPtr,
    out_func: ConstPtr,
    flush_func: ConstPtr,
) -> LinkPtr {
    if out_func.is_null() || flush_func.is_null() {
        return log;
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
        take_link!(log, LogImpl<DynamicTransducer, Box<L>>)
            .with_logger(get_logger_with_custom_func(out_func, flush_func)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use autd3capi_def::Level;

    use crate::link::debug::*;

    #[test]
    fn test_link_debug() {
        unsafe {
            let link = AUTDLinkDebug();
            let link = AUTDLinkLog(link);
            let link = AUTDLinkLogWithLogLevel(link, Level::Debug);

            let out_f = |_msg: *const c_char| {};
            let flush_f = || {};

            let _link =
                AUTDLinkLogWithLogFunc(link, &out_f as *const _ as _, &flush_f as *const _ as _);
        }
    }
}
