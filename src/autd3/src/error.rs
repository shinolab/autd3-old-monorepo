/*
 * File: error.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::error::AUTDInternalError;
use thiserror::Error;

pub struct ReadFirmwareInfoState(pub Vec<bool>);

impl std::fmt::Display for ReadFirmwareInfoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Read firmware info failed: {}",
            self.0
                .iter()
                .enumerate()
                .filter_map(|(i, b)| if *b { None } else { Some(i.to_string()) })
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl std::fmt::Debug for ReadFirmwareInfoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

#[derive(Error, Debug)]
pub enum AUTDError {
    #[error("Device id ({0}) is specified, but only {1} AUTDs are connected.")]
    GroupedOutOfRange(usize, usize),
    #[error("{0}")]
    ReadFirmwareInfoFailed(ReadFirmwareInfoState),
    #[error("{0}")]
    Internal(AUTDInternalError),
}

impl From<AUTDInternalError> for AUTDError {
    fn from(e: AUTDInternalError) -> Self {
        AUTDError::Internal(e)
    }
}
