/*
 * File: sync_mode.rs
 * Project: src
 * Created Date: 08/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncMode {
    DC,
    FreeRun,
}
