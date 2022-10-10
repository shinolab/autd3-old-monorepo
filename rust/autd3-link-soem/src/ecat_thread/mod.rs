/*
 * File: mod.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod error_handler;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(all(unix, not(target_os = "macos")))]
mod unix;
mod utils;
#[cfg(windows)]
mod win32;

#[cfg(windows)]
pub use win32::*;

#[cfg(all(unix, not(target_os = "macos")))]
pub use unix::*;

#[cfg(target_os = "macos")]
pub use macos::*;

pub use error_handler::EcatErrorHandler;
