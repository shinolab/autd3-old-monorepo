/*
 * File: error.rs
 * Project: src
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::error::AUTDInternalError;
use thiserror::Error;

#[cfg(feature = "plotters")]
use plotters::drawing::DrawingAreaErrorKind;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[cfg(feature = "python")]
    #[error("{0}")]
    PyO3Error(pyo3::PyErr),
    #[error("Plot range is invalid")]
    InvalidPlotRange,
    #[cfg(feature = "plotters")]
    #[error("{0}")]
    DrawingAreaError(String),
    #[error("Not supported operation")]
    NotSupported,
    #[error("{0}")]
    IoError(std::io::Error),
    #[cfg(feature = "plotters")]
    #[error("{0}")]
    BitMapBackendError(plotters_bitmap::BitMapBackendError),
}

impl From<MonitorError> for AUTDInternalError {
    fn from(val: MonitorError) -> AUTDInternalError {
        AUTDInternalError::LinkError(val.to_string())
    }
}

#[cfg(feature = "plotters")]
impl<E: std::error::Error + Send + Sync> From<DrawingAreaErrorKind<E>> for MonitorError {
    fn from(value: DrawingAreaErrorKind<E>) -> Self {
        Self::DrawingAreaError(value.to_string())
    }
}

#[cfg(feature = "plotters")]
impl From<plotters_bitmap::BitMapBackendError> for MonitorError {
    fn from(value: plotters_bitmap::BitMapBackendError) -> Self {
        Self::BitMapBackendError(value)
    }
}

#[cfg(feature = "python")]
impl From<pyo3::PyErr> for MonitorError {
    fn from(value: pyo3::PyErr) -> Self {
        Self::PyO3Error(value)
    }
}

impl From<std::io::Error> for MonitorError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
