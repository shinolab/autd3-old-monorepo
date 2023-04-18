/*
 * File: option.rs
 * Project: src
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{SyncMode, TimerStrategy};

pub struct Config {
    pub sync0_cycle: u16,
    pub send_cycle: u16,
    pub buf_size: usize,
    pub timer_strategy: TimerStrategy,
    pub sync_mode: SyncMode,
    pub ifname: String,
    pub state_check_interval: std::time::Duration,
    pub timeout: std::time::Duration,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sync0_cycle: 2,
            send_cycle: 2,
            buf_size: 32,
            timer_strategy: TimerStrategy::default(),
            sync_mode: SyncMode::default(),
            ifname: String::new(),
            state_check_interval: std::time::Duration::from_millis(100),
            timeout: std::time::Duration::from_millis(20),
        }
    }
}
