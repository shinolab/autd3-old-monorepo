/*
 * File: macosx.rs
 * Project: src
 * Created Date: 24/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use crate::error::AUTDInternalError;
use libc::{c_char, c_long, c_ulong, c_void, uintptr_t};
use std::ffi::CString;

#[allow(non_camel_case_types)]
type dispatch_object_t = *const c_void;
#[allow(non_camel_case_types)]
type dispatch_queue_t = *const c_void;
#[allow(non_camel_case_types)]
type dispatch_source_t = *const c_void;
#[allow(non_camel_case_types)]
type dispatch_source_type_t = *const c_void;
#[allow(non_camel_case_types)]
type dispatch_time_t = u64;

const DISPATCH_TIME_NOW: dispatch_time_t = 0;

type Waitortimercallback = unsafe extern "C" fn(*const c_void);

extern "C" {
    static _dispatch_source_type_timer: c_long;
    fn dispatch_queue_create(label: *const c_char, attr: c_ulong) -> dispatch_queue_t;
    fn dispatch_source_create(
        type_: dispatch_source_type_t,
        handle: uintptr_t,
        mask: c_ulong,
        queue: dispatch_queue_t,
    ) -> dispatch_source_t;
    fn dispatch_source_set_timer(
        source: dispatch_source_t,
        start: dispatch_time_t,
        interval: u64,
        leeway: u64,
    );
    fn dispatch_source_set_event_handler_f(source: dispatch_source_t, handler: Waitortimercallback);
    fn dispatch_resume(object: dispatch_object_t);
    fn dispatch_release(object: dispatch_object_t);
    fn dispatch_time(when: dispatch_time_t, delta: i64) -> dispatch_time_t;
    fn dispatch_set_context(obj: dispatch_object_t, context: *const c_void);
}

struct TimerHandle {
    timer: dispatch_source_t,
}

unsafe impl Send for TimerHandle {}
unsafe impl Sync for TimerHandle {}

pub struct NativeTimerWrapper {
    timer_handle: Option<TimerHandle>,
}

impl NativeTimerWrapper {
    pub fn new() -> NativeTimerWrapper {
        NativeTimerWrapper { timer_handle: None }
    }

    pub fn start<P>(
        &mut self,
        cb: Option<Waitortimercallback>,
        period: std::time::Duration,
        lp_param: *mut P,
    ) -> Result<bool, AUTDInternalError> {
        unsafe {
            let timer_queue_str = CString::new("timerQueue").unwrap();
            let queue = dispatch_queue_create(timer_queue_str.as_ptr(), 0);
            let timer = dispatch_source_create(
                &_dispatch_source_type_timer as *const _ as dispatch_source_type_t,
                0,
                0,
                queue,
            );
            dispatch_set_context(timer, lp_param as *const c_void);

            dispatch_source_set_event_handler_f(timer, cb.unwrap());

            let start = dispatch_time(DISPATCH_TIME_NOW, 0);
            dispatch_source_set_timer(timer, start, period.as_nanos() as u64, 0);
            dispatch_resume(timer);

            self.timer_handle = Some(TimerHandle { timer });

            Ok(true)
        }
    }

    pub fn close(&mut self) -> Result<(), AUTDInternalError> {
        if let Some(handle) = self.timer_handle.take() {
            unsafe {
                dispatch_release(handle.timer);
            }
        }
        Ok(())
    }
}

impl Drop for NativeTimerWrapper {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
