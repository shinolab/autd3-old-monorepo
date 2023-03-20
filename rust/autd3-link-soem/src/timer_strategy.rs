/*
 * File: timer_strategy.rs
 * Project: src
 * Created Date: 21/03/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TimerStrategy {
    #[default]
    Sleep,
    BusyWait,
    NativeTimer,
}
