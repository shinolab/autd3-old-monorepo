/*
 * File: error.rs
 * Project: src
 * Created Date: 15/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::error::AUTDInternalError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioFileError {
    #[error("{0}")]
    Io(std::io::Error),
    #[error("{0}")]
    Wav(hound::Error),
}

impl From<std::io::Error> for AudioFileError {
    fn from(e: std::io::Error) -> Self {
        AudioFileError::Io(e)
    }
}

impl From<hound::Error> for AudioFileError {
    fn from(e: hound::Error) -> Self {
        AudioFileError::Wav(e)
    }
}

impl From<AudioFileError> for AUTDInternalError {
    fn from(value: AudioFileError) -> Self {
        AUTDInternalError::ModulationError(value.to_string())
    }
}
