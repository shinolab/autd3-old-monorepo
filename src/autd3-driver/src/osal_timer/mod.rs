/*
 * File: mod.rs
 * Project: osal_timer
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use self::windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use self::linux::*;

#[cfg(target_os = "macos")]
mod macosx;
#[cfg(target_os = "macos")]
use self::macosx::*;

mod timer;

pub use timer::{Timer, TimerCallback};
