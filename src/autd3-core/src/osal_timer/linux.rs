/*
 * File: linux.rs
 * Project: src
 * Created Date: 24/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use super::error::TimerError;
use libc::{
    c_int, c_void, clockid_t, itimerspec, sigaction, sigevent, siginfo_t, timespec, CLOCK_REALTIME,
};
use std::{mem, ptr};

#[allow(non_camel_case_types)]
type timer_t = usize;

extern "C" {
    fn timer_create(clockid_t: clockid_t, sevp: *mut sigevent, timerid: *mut timer_t) -> c_int;
    fn timer_settime(
        timerid: timer_t,
        flags: c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> c_int;
    fn timer_delete(timerid: timer_t) -> c_int;
}

const SIGRTMIN: c_int = 34;
type Waitortimercallback = unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void);

struct TimerHandle {
    timer: timer_t,
}

unsafe impl Send for TimerHandle {}

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
    ) -> Result<bool, TimerError> {
        unsafe {
            let mut sa: sigaction = mem::zeroed();
            sa.sa_flags = libc::SA_SIGINFO;
            sa.sa_sigaction = cb.unwrap() as usize;
            libc::sigemptyset(&mut sa.sa_mask);
            if sigaction(SIGRTMIN, &sa, ptr::null_mut()) < 0 {
                return Err(TimerError::CreationFailed());
            }

            let mut sev: sigevent = mem::zeroed();
            sev.sigev_value = libc::sigval {
                sival_ptr: lp_param as *mut c_void,
            };
            sev.sigev_signo = SIGRTMIN;

            sev.sigev_notify = libc::SIGEV_THREAD_ID;
            let tid = libc::syscall(libc::SYS_gettid);
            sev.sigev_notify_thread_id = tid as i32;

            let mut timer = 0;
            if timer_create(CLOCK_REALTIME, &mut sev, &mut timer) < 0 {
                return Err(TimerError::CreationFailed());
            }

            let new_value = itimerspec {
                it_interval: timespec {
                    tv_sec: period.as_secs() as _,
                    tv_nsec: period.subsec_nanos() as _,
                },
                it_value: timespec {
                    tv_sec: period.as_secs() as _,
                    tv_nsec: period.subsec_nanos() as _,
                },
            };

            if timer_settime(timer, 0, &new_value, ptr::null_mut()) < 0 {
                return Err(TimerError::CreationFailed());
            }

            self.timer_handle = Some(TimerHandle { timer });
            Ok(true)
        }
    }

    pub fn close(&mut self) -> Result<(), TimerError> {
        if let Some(handle) = self.timer_handle.take() {
            unsafe {
                if timer_delete(handle.timer) < 0 {
                    return Err(TimerError::DeleteFailed());
                }
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
