/*
 * File: error.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::error::AUTDInternalError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AUTDError {
    #[error("Device id ({0}) is specified, but only {1} AUTDs are connected.")]
    GroupedOutOfRange(usize, usize),
    #[error("{0}")]
    Internal(AUTDInternalError),
}

impl From<AUTDInternalError> for AUTDError {
    fn from(e: AUTDInternalError) -> Self {
        AUTDError::Internal(e)
    }
}
