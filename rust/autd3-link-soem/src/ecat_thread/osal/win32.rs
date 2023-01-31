/*
 * File: win32.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use libc::timespec;
use once_cell::sync::Lazy;

use windows::Win32::{
    Networking::WinSock::TIMEVAL,
    System::{
        Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
        SystemInformation::GetSystemTimePreciseAsFileTime,
    },
};

use crate::ecat_thread::waiter::Waiter;

static PERFORMANCE_FREQUENCY: Lazy<i64> = Lazy::new(|| unsafe {
    let mut freq = 0;
    QueryPerformanceFrequency(&mut freq as *mut _);
    freq
});

fn osal_gettimeofday(tv: *mut TIMEVAL) {
    unsafe {
        let system_time = GetSystemTimePreciseAsFileTime();

        let mut system_time64 =
            ((system_time.dwHighDateTime as i64) << 32) + system_time.dwLowDateTime as i64;
        system_time64 += -134774i64 * 86400i64 * 1000000i64 * 10i64;
        let usecs = system_time64 / 10;

        (*tv).tv_sec = (usecs / 1000000) as _;
        (*tv).tv_usec = (usecs - ((*tv).tv_sec as i64 * 1000000i64)) as _;
    }
}

fn nanosleep(t: i64) {
    unsafe {
        let mut start = 0;
        QueryPerformanceCounter(&mut start as *mut _);

        let pf = *PERFORMANCE_FREQUENCY;
        let sleep = t * pf / (1000 * 1000 * 1000);
        loop {
            let mut now = 0;
            QueryPerformanceCounter(&mut now as *mut _);
            if now - start > sleep {
                break;
            }
        }
    }
}

pub fn ecat_setup(cycletime_ns: i64) -> timespec {
    let mut tp = TIMEVAL {
        tv_sec: 0,
        tv_usec: 0,
    };
    osal_gettimeofday(&mut tp as *mut _);
    let cycletime_us = (cycletime_ns / 1000i64) as i32;

    let ht = (tp.tv_usec / cycletime_us + 1) * cycletime_us;
    timespec {
        tv_sec: tp.tv_sec as _,
        tv_nsec: ht * 1000i32,
    }
}

pub fn add_timespec(ts: &mut timespec, addtime: i64) {
    let nsec = addtime % 1000000000;
    let sec = (addtime - nsec) / 1000000000;
    ts.tv_sec += sec;
    ts.tv_nsec += nsec as i32;
    if ts.tv_nsec >= 1000000000 {
        let nsec = ts.tv_nsec % 1000000000;
        ts.tv_sec += ((ts.tv_nsec - nsec) / 1000000000) as i64;
        ts.tv_nsec = nsec;
    }
}

pub struct NormalWaiter {}
pub struct HighPrecisionWaiter {}

impl Waiter for NormalWaiter {
    fn timed_wait(abs_time: &timespec) {
        let mut tp = TIMEVAL {
            tv_sec: 0,
            tv_usec: 0,
        };
        osal_gettimeofday(&mut tp as *mut _);

        let sleep = (abs_time.tv_sec - tp.tv_sec as i64) * 1000000000
            + (abs_time.tv_nsec - tp.tv_usec * 1000) as i64;

        if sleep > 0 {
            std::thread::sleep(std::time::Duration::from_nanos(sleep as _));
        }
    }
}

impl Waiter for HighPrecisionWaiter {
    fn timed_wait(abs_time: &timespec) {
        let mut tp = TIMEVAL {
            tv_sec: 0,
            tv_usec: 0,
        };
        osal_gettimeofday(&mut tp as *mut _);

        let sleep = (abs_time.tv_sec - tp.tv_sec as i64) * 1000000000
            + (abs_time.tv_nsec - tp.tv_usec * 1000) as i64;

        nanosleep(sleep);
    }
}
