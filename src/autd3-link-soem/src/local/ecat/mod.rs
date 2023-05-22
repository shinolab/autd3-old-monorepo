/*
 * File: mod.rs
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

#[cfg(windows)]
mod osal {
    mod win32;
    pub use libc::timespec;
    pub use win32::*;
}
#[cfg(target_os = "macos")]
mod osal {
    mod macos;
    pub use libc::gettimeofday;
    pub use macos::*;
}
#[cfg(all(unix, not(target_os = "macos")))]
mod osal {
    mod unix;
    pub use libc::gettimeofday;
    pub use unix::*;
}

pub use osal::*;

pub fn ec_sync(reftime: i64, cycletime: i64, integral: &mut i64) -> i64 {
    let mut delta = (reftime - 50000) % cycletime;
    if delta > (cycletime / 2) {
        delta -= cycletime;
    }
    if delta > 0 {
        *integral += 1;
    }
    if delta < 0 {
        *integral -= 1;
    }
    -(delta / 100) - (*integral / 20)
}
