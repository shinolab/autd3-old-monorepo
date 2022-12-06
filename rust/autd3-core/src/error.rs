/*
 * File: error.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

use autd3_driver::MAX_CYCLE;

#[derive(Error, Debug)]
pub enum AUTDInternalError {
    #[error("Link is closed.")]
    LinkClosed,
    #[error("Maximum cycle is {} , but {0} is specified", MAX_CYCLE)]
    CycleOutOfRange(u16),
    #[error("The maximum number of transducers per device is 256")]
    TransducersNumInDeviceOutOfRange,
}
