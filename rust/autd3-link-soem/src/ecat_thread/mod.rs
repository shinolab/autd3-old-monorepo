/*
 * File: mod.rs
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

mod ecat_thread;
mod error_handler;
mod utils;
mod waiter;

#[cfg(windows)]
mod osal {
    mod win32;
    pub use win32::*;
}
#[cfg(target_os = "macos")]
mod osal {
    mod macos;
    pub use macos::*;
}
#[cfg(all(unix, not(target_os = "macos")))]
mod osal {
    mod unix;
    pub use unix::*;
}

pub use error_handler::EcatErrorHandler;
