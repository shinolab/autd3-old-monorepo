/*
 * File: sync_mode.rs
 * Project: src
 * Created Date: 12/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use serde::{Deserialize, Serialize};

/// Synchronization modes of an EtherCAT slave
/// See [Beckhoff's document](https://infosys.beckhoff.com/english.php?content=../content/1033/ethercatsystem/2469122443.html&id=) for more details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Deserialize, Serialize)]
pub enum SyncMode {
    /// DC sync mode
    DC,
    /// Free run mode
    #[default]
    FreeRun,
}
