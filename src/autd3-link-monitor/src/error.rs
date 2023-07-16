/*
 * File: error.rs
 * Project: src
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::error::AUTDInternalError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[cfg(feature = "python")]
    #[error("{0}")]
    PyO3Error(pyo3::PyErr),
    #[error("Plot range is invalid")]
    InvalidPlotRange,
}

impl From<MonitorError> for AUTDInternalError {
    fn from(val: MonitorError) -> AUTDInternalError {
        AUTDInternalError::LinkError(val.to_string())
    }
}

#[cfg(feature = "python")]
impl From<pyo3::PyErr> for MonitorError {
    fn from(value: pyo3::PyErr) -> Self {
        Self::PyO3Error(value)
    }
}
