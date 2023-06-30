/*
 * File: error.rs
 * Project: src
 * Created Date: 30/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::error::Error;

use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum AUTDProtoBufError {
    #[error("{0}")]
    Status(String),
    #[error("{0}")]
    SendError(String),
    #[error("{0}")]
    TokioSendError(String),
    #[error("{0}")]
    TransportError(String),
    #[error("{0}")]
    TokioJoinError(String),
    #[error("{0}")]
    AUTDError(autd3::prelude::AUTDError),
    #[error("This data is not supported.")]
    NotSupportedData,
}

impl From<autd3::prelude::AUTDError> for AUTDProtoBufError {
    fn from(e: autd3::prelude::AUTDError) -> Self {
        AUTDProtoBufError::AUTDError(e)
    }
}

impl From<autd3_core::error::AUTDInternalError> for AUTDProtoBufError {
    fn from(e: autd3_core::error::AUTDInternalError) -> Self {
        AUTDProtoBufError::AUTDError(e.into())
    }
}

impl From<tonic::Status> for AUTDProtoBufError {
    fn from(e: tonic::Status) -> Self {
        AUTDProtoBufError::Status(e.to_string())
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for AUTDProtoBufError {
    fn from(e: std::sync::mpsc::SendError<T>) -> Self {
        AUTDProtoBufError::SendError(e.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for AUTDProtoBufError {
    fn from(e: tokio::sync::mpsc::error::SendError<T>) -> Self {
        AUTDProtoBufError::TokioSendError(e.to_string())
    }
}

impl From<tokio::task::JoinError> for AUTDProtoBufError {
    fn from(e: tokio::task::JoinError) -> Self {
        AUTDProtoBufError::TokioJoinError(e.to_string())
    }
}

impl From<tonic::transport::Error> for AUTDProtoBufError {
    fn from(e: tonic::transport::Error) -> Self {
        match e.source() {
            Some(source) => AUTDProtoBufError::TransportError(source.to_string()),
            None => AUTDProtoBufError::TransportError(e.to_string()),
        }
    }
}

impl From<AUTDProtoBufError> for autd3_core::error::AUTDInternalError {
    fn from(e: AUTDProtoBufError) -> Self {
        autd3_core::error::AUTDInternalError::LinkError(e.to_string())
    }
}

pub fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;
    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }
        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}
