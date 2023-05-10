/*
 * File: error.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/05/2023
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
    #[error("Failed to open link: {0}")]
    LinkOpenFailed(&'static str),
    #[error("Maximum cycle is {} , but {0} is specified", MAX_CYCLE)]
    CycleOutOfRange(u16),
    #[error("The maximum number of transducers per device is 256")]
    TransducersNumInDeviceOutOfRange,
}
