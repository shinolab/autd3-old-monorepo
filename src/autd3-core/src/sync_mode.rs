/*
 * File: sync_mode.rs
 * Project: src
 * Created Date: 12/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

 use serde::{Deserialize, Serialize};

 #[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Deserialize, Serialize)]
 pub enum SyncMode {
     DC,
     #[default]
     FreeRun,
 }
 