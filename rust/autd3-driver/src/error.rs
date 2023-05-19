/*
 * File: error.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

use crate::FPGA_SUB_CLK_FREQ_DIV;

#[derive(Error, Debug)]
pub enum DriverError {
    #[error(
        "{} transducer{} connected, but you try to use {}", a, if *a == 1 {" is"} else {"s are"}, b)]
    NumberOfTransducerMismatch { a: usize, b: usize },
    #[error(
        "Maximum size is {}, but {0} data are to be send",
        crate::MOD_BUF_SIZE_MAX
    )]
    ModulationSizeOutOfRange(usize),
    #[error(
        "Minimum modulation frequency division is {}, but {0} is used",
        crate::SAMPLING_FREQ_DIV_MIN / FPGA_SUB_CLK_FREQ_DIV as u32
    )]
    ModFreqDivOutOfRange(u32),
    #[error("STM index is out of range")]
    STMStartIndexOutOfRange,
    #[error("STM finish is out of range")]
    STMFinishIndexOutOfRange,
    #[error("Maximum size is {}, but {0} is used", crate::FOCUS_STM_BUF_SIZE_MAX)]
    FocusSTMPointSizeOutOfRange(usize),
    #[error(
        "Minimum FocusSTM frequency division is {}, but {0} is used",
        crate::SAMPLING_FREQ_DIV_MIN / FPGA_SUB_CLK_FREQ_DIV as u32
    )]
    FocusSTMFreqDivOutOfRange(u32),
    #[error(
        "Maximum size is {}, but {0} is used",
        crate::GAIN_STM_LEGACY_BUF_SIZE_MAX
    )]
    GainSTMLegacySizeOutOfRange(usize),
    #[error("Maximum size is {}, but {0} is used", crate::GAIN_STM_BUF_SIZE_MAX)]
    GainSTMSizeOutOfRange(usize),
    #[error(
        "Minimum GainSTM frequency division is {}, but {0} is used",
        crate::SAMPLING_FREQ_DIV_MIN / FPGA_SUB_CLK_FREQ_DIV as u32
    )]
    GainSTMLegacyFreqDivOutOfRange(u32),
    #[error(
        "Minimum GainSTM frequency division is {}, but {0} is used",
        crate::SAMPLING_FREQ_DIV_MIN / FPGA_SUB_CLK_FREQ_DIV as u32
    )]
    GainSTMFreqDivOutOfRange(u32),
    #[error("PhaseHalf is not supported in Advanced mode")]
    PhaseHalfNotSupported,
}
