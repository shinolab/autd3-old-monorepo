/*
 * File: waiter.rs
 * Project: ecat_thread
 * Created Date: 04/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use libc::timespec;

pub trait Waiter {
    fn timed_wait(abs_time: &timespec);
}
