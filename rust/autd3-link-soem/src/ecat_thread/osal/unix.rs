/*
 * File: unix.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::ecat_thread::waiter::Waiter;
use libc::{clock_gettime, clock_nanosleep, timespec, CLOCK_MONOTONIC, TIMER_ABSTIME};

pub struct NormalWaiter {}
pub struct HighPrecisionWaiter {}

pub fn add_timespec(ts: &mut timespec, addtime: i64) {
    let nsec = addtime % 1000000000;
    let sec = (addtime - nsec) / 1000000000;
    ts.tv_sec += sec;
    ts.tv_nsec += nsec;
    if ts.tv_nsec >= 1000000000 {
        let nsec = ts.tv_nsec % 1000000000;
        ts.tv_sec += ((ts.tv_nsec - nsec) / 1000000000) as i64;
        ts.tv_nsec = nsec;
    }
}

pub fn ecat_setup(cycletime_ns: i64) -> timespec {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut tleft = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    clock_gettime(CLOCK_MONOTONIC, &mut ts as *mut _);

    let ht = ((ts.tv_nsec / cycletime_ns) + 1) * cycletime_ns;
    ts.tv_nsec = ht;

    ts
}

impl Waiter for NormalWaiter {
    fn timed_wait(abs_time: &timespec) {
        let tleft = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_nanosleep(
            CLOCK_MONOTONIC,
            TIMER_ABSTIME,
            abs_time,
            &mut tleft as *mut _,
        );
    }
}

impl Waiter for HighPrecisionWaiter {
    fn timed_wait(abs_time: &timespec) {
        let tleft = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_nanosleep(
            CLOCK_MONOTONIC,
            TIMER_ABSTIME,
            abs_time,
            &mut tleft as *mut _,
        );
    }
}
