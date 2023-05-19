/*
 * File: win32.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use libc::{timespec, timezone};

use windows::Win32::{
    Networking::WinSock::TIMEVAL, System::SystemInformation::GetSystemTimePreciseAsFileTime,
};

pub fn gettimeofday(tv: *mut TIMEVAL, _tz: *const timezone) {
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

pub fn ecat_setup(cycletime_ns: i64) -> timespec {
    let mut tp = TIMEVAL {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&mut tp as *mut _, std::ptr::null());
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
