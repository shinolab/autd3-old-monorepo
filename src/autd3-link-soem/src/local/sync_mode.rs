/*
 * File: sync_mode.rs
 * Project: src
 * Created Date: 08/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Deserialize, Serialize)]
pub enum SyncMode {
    DC,
    #[default]
    FreeRun,
}
