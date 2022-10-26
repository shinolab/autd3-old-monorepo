/*
 * File: option.rs
 * Project: src
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::SyncMode;

pub struct Config {
    pub sync0_cycle: u16,
    pub send_cycle: u16,
    pub high_precision_timer: bool,
    pub sync_mode: SyncMode,
    pub ifname: String,
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
            high_precision_timer: false,
            sync_mode: SyncMode::DC,
            ifname: String::new(),
        }
    }
}
