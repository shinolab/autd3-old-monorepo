/*
 * File: error.rs
 * Project: src
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::error::AUTDInternalError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HoloError {
    #[error("Failed to solve linear system")]
    SolveFailed,
    #[error("{0}")]
    BackendError(String),
    #[error("{0}")]
    BackendCreationError(String),
}

impl From<HoloError> for AUTDInternalError {
    fn from(value: HoloError) -> Self {
        AUTDInternalError::GainError(value.to_string())
    }
}
