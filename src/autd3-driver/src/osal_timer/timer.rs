/*
 * File: timer.rs
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

use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::AUTDInternalError;

use super::NativeTimerWrapper;
#[cfg(target_os = "macos")]
use libc::c_void;
#[cfg(target_os = "linux")]
use libc::{c_int, c_void, siginfo_t};

pub trait TimerCallback {
    fn rt_thread(&mut self);
}

pub struct Timer<F: TimerCallback> {
    lock: AtomicBool,
    native_timer: NativeTimerWrapper,
    cb: F,
}

impl<F: TimerCallback> Timer<F> {
    pub fn start(cb: F, period: std::time::Duration) -> Result<Box<Self>, AUTDInternalError> {
        let mut timer = Box::new(Self {
            lock: AtomicBool::new(false),
            native_timer: NativeTimerWrapper::new(),
            cb,
        });
        let ptr = &mut *timer as *mut Self;
        timer
            .native_timer
            .start(Some(Self::rt_thread), period, ptr)?;
        Ok(timer)
    }

    pub fn close(mut self) -> Result<F, AUTDInternalError> {
        self.native_timer.close()?;
        Ok(self.cb)
    }

    #[cfg(target_os = "windows")]
    unsafe extern "system" fn rt_thread(
        _u_timer_id: u32,
        _u_msg: u32,
        dw_user: usize,
        _dw1: usize,
        _dw2: usize,
    ) {
        let ptr = dw_user as *mut Self;
        if let Some(timer) = ptr.as_mut() {
            if let Ok(false) =
                timer
                    .lock
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            {
                timer.cb.rt_thread();
                timer.lock.store(false, Ordering::Release);
            }
        }
    }

    #[cfg(target_os = "linux")]
    unsafe extern "C" fn rt_thread(_sig: c_int, si: *mut siginfo_t, _uc: *mut c_void) {
        let ptr = Self::get_ptr(si);
        let ptr = ptr as *mut Self;
        if let Some(timer) = ptr.as_mut() {
            if let Ok(false) =
                timer
                    .lock
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            {
                timer.cb.rt_thread();
                timer.lock.store(false, Ordering::Release);
            }
        }
    }

    #[cfg(target_os = "linux")]
    #[allow(deprecated)]
    unsafe extern "C" fn get_ptr(si: *mut siginfo_t) -> u64 {
        // TODO: This depends on the deprecated field of libc crate, and may only work on a specific platforms.
        let ptr_lsb = (*si)._pad[3];
        let ptr_msb = (*si)._pad[4];
        ((ptr_msb as u64) << 32) | (ptr_lsb as u64 & 0xFFFF_FFFF)
    }

    #[cfg(target_os = "macos")]
    unsafe extern "C" fn rt_thread(ptr: *const c_void) {
        let ptr = ptr as *mut Self;
        if let Some(timer) = ptr.as_mut() {
            if let Ok(false) =
                timer
                    .lock
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            {
                timer.cb.rt_thread();
                timer.lock.store(false, Ordering::Release);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct CountCallback {
        count: usize,
    }

    impl TimerCallback for CountCallback {
        fn rt_thread(&mut self) {
            self.count += 1;
        }
    }

    #[test]
    fn test_timer() {
        let timer = Timer::start(
            CountCallback { count: 0 },
            std::time::Duration::from_millis(1),
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let count = timer.cb.count;
        assert!(90 < count && count < 110);
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(180 < timer.cb.count && timer.cb.count < 220);
        let cb = timer.close().unwrap();
        let count = cb.count;
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(cb.count, count);
    }
}
