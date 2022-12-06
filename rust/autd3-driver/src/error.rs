/*
 * File: error.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

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
        crate::MOD_SAMPLING_FREQ_DIV_MIN
    )]
    ModFreqDivOutOfRange(u32),
    #[error(
        "Minimum silencer cycle is {}, but {0} is used",
        crate::SILENCER_CYCLE_MIN
    )]
    SilencerCycleOutOfRange(u16),
    #[error("Maximum size is {}, but {0} is used", crate::FOCUS_STM_BUF_SIZE_MAX)]
    FocusSTMPointSizeOutOfRange(usize),
    #[error(
        "FocusSTM frequency division is {}, but {0} is used",
        crate::FOCUS_STM_SAMPLING_FREQ_DIV_MIN
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
        "GainSTM frequency division is {}, but {0} is used",
        crate::GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN
    )]
    GainSTMLegacyFreqDivOutOfRange(u32),
    #[error(
        "GainSTM frequency division is {}, but {0} is used",
        crate::GAIN_STM_SAMPLING_FREQ_DIV_MIN
    )]
    GainSTMFreqDivOutOfRange(u32),
    #[error("PhaseHalf is not supported in Normal mode")]
    PhaseHalfNotSupported,
}
