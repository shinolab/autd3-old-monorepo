/*
 * File: macos.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use libc::{gettimeofday, timespec};

pub fn ecat_setup(cycletime_ns: i64) -> timespec {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        gettimeofday(&mut ts as *mut _ as *mut _, std::ptr::null_mut() as *mut _);
    }
    let ht = ((ts.tv_nsec / cycletime_ns) + 1) * cycletime_ns;
    ts.tv_nsec = ht;

    ts
}

pub fn add_timespec(ts: &mut timespec, addtime: i64) {
    let nsec = addtime % 1000000000;
    let sec = (addtime - nsec) / 1000000000;
    ts.tv_sec += sec;
    ts.tv_nsec += nsec;
    if ts.tv_nsec >= 1000000000 {
        let nsec = ts.tv_nsec % 1000000000;
        ts.tv_sec += (ts.tv_nsec - nsec) / 1000000000;
        ts.tv_nsec = nsec;
    }
}
