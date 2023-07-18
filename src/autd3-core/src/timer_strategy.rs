/*
 * File: timer_strategy.rs
 * Project: src
 * Created Date: 08/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

/// Timer strategy
#[derive(
    FromPrimitive,
    ToPrimitive,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Deserialize,
    Serialize,
)]
#[repr(u8)]
pub enum TimerStrategy {
    /// Use `std::thread::sleep`
    Sleep = 0,
    /// Use busy wait
    BusyWait = 1,
    /// Use OS-native timer
    NativeTimer = 2,
}
