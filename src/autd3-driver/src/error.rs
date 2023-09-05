/*
 * File: error.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

use crate::{fpga::*, operation::GainSTMMode};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AUTDInternalError {
    #[error(
        "{} transducer{} connected, but you try to use {}", a, if *a == 1 {" is"} else {"s are"}, b)]
    NumberOfTransducerMismatch { a: usize, b: usize },
    #[error("Maximum size is {}, but {0} data are to be send", MOD_BUF_SIZE_MAX)]
    ModulationSizeOutOfRange(usize),
    #[error(
        "Frequency division must be in [{}, {}], but {0} is used",
        SAMPLING_FREQ_DIV_MIN, u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32
    )]
    ModFreqDivOutOfRange(u32),
    #[error("STM index is out of range")]
    STMStartIndexOutOfRange,
    #[error("STM finish is out of range")]
    STMFinishIndexOutOfRange,
    #[error("Maximum size is {}, but {0} is used", FOCUS_STM_BUF_SIZE_MAX)]
    FocusSTMPointSizeOutOfRange(usize),
    #[error(
        "Frequency division must be in [{}, {}], but {0} is used",
        SAMPLING_FREQ_DIV_MIN, u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32
    )]
    FocusSTMFreqDivOutOfRange(u32),
    #[error("Maximum size is {}, but {0} is used", GAIN_STM_LEGACY_BUF_SIZE_MAX)]
    GainSTMLegacySizeOutOfRange(usize),
    #[error("Maximum size is {}, but {0} is used", GAIN_STM_BUF_SIZE_MAX)]
    GainSTMSizeOutOfRange(usize),
    #[error(
        "Frequency division must be in [{}, {}], but {0} is used",
        SAMPLING_FREQ_DIV_MIN, u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32
    )]
    GainSTMLegacyFreqDivOutOfRange(u32),
    #[error(
        "Frequency division must be in [{}, {}], but {0} is used",
        SAMPLING_FREQ_DIV_MIN, u32::MAX / FPGA_SUB_CLK_FREQ_DIV as u32
    )]
    GainSTMFreqDivOutOfRange(u32),
    #[error("PhaseHalf is not supported in Advanced mode")]
    PhaseHalfNotSupported,
    #[error("Maximum cycle is {} , but {0} is specified", MAX_CYCLE)]
    CycleOutOfRange(u16),

    #[error("GainSTMMode ({0}) is not supported")]
    GainSTMModeNotSupported(GainSTMMode),

    #[error("Unknown group key")]
    UnknownGroupKey,
    #[error("Unspecified group key")]
    UnspecifiedGroupKey,

    #[error("{0}")]
    ModulationError(String),
    #[error("{0}")]
    GainError(String),
    #[error("{0}")]
    LinkError(String),

    #[error("failed to create timer")]
    TimerCreationFailed(),
    #[error("failed to delete timer")]
    TimerDeleteFailed(),
}
