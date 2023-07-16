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

use plotters::drawing::DrawingAreaErrorKind;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[cfg(feature = "python")]
    #[error("{0}")]
    PyO3Error(pyo3::PyErr),
    #[error("Plot range is invalid")]
    InvalidPlotRange,
    #[error("{0}")]
    BitmapDrawingAreaError(
        DrawingAreaErrorKind<
            <plotters::prelude::BitMapBackend<'static> as plotters::prelude::DrawingBackend>::ErrorType,
        >,
    ),
}

impl From<MonitorError> for AUTDInternalError {
    fn from(val: MonitorError) -> AUTDInternalError {
        AUTDInternalError::LinkError(val.to_string())
    }
}

impl From<
DrawingAreaErrorKind<
<plotters::prelude::BitMapBackend<'static> as plotters::prelude::DrawingBackend>::ErrorType,
> 
> for MonitorError {
    fn from(value: DrawingAreaErrorKind<
        <plotters::prelude::BitMapBackend<'static> as plotters::prelude::DrawingBackend>::ErrorType,
        >) -> Self {
        Self::BitmapDrawingAreaError(value)
    }
}

#[cfg(feature = "python")]
impl From<pyo3::PyErr> for MonitorError {
    fn from(value: pyo3::PyErr) -> Self {
        Self::PyO3Error(value)
    }
}
