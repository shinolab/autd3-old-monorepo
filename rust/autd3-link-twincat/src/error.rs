/*
 * File: error.rs
 * Project: src
 * Created Date: 27/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::error::AUTDInternalError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdsError {
    #[error("Failed to open port")]
    OpenPort,
    #[error("Failed to get local address: {0}")]
    GetLocalAddress(i32),
    #[error("Failed to send data: {0}")]
    SendData(i32),
    #[error("Failed to read data: {0}")]
    ReadData(i32),
}

impl From<AdsError> for AUTDInternalError {
    fn from(err: AdsError) -> Self {
        AUTDInternalError::LinkError(err.to_string())
    }
}
