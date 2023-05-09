/*
 * File: error.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, DriverError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AUTDError {
    #[error("Device id ({0}) is specified, but only {1} AUTDs are connected.")]
    GroupedOutOfRange(usize, usize),
    #[error("{0}")]
    Driver(DriverError),
    #[error("{0}")]
    Internal(AUTDInternalError),
}

impl From<DriverError> for AUTDError {
    fn from(e: DriverError) -> Self {
        AUTDError::Driver(e)
    }
}

impl From<AUTDInternalError> for AUTDError {
    fn from(e: AUTDInternalError) -> Self {
        AUTDError::Internal(e)
    }
}
