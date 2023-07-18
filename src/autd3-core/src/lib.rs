/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod acoustics;
pub mod amplitude;
pub mod autd3_device;
pub mod clear;
pub mod datagram;
pub mod delay;
pub mod error;
pub mod gain;
pub mod geometry;
pub mod link;
pub mod modulation;
#[doc(hidden)]
pub mod osal_timer;
pub mod silencer_config;
pub mod stm;
pub mod stop;
pub mod sync_mode;
pub mod synchronize;
pub mod timer_strategy;
pub mod update_flag;

pub use autd3_driver::*;

pub use spdlog;
