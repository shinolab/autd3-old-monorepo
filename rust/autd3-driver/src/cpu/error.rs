/*
 * File: error.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CPUError {
    #[error("{} device{} connected, but {} {} specified", a, if *a == 1 {" is"} else {"s are"}, b, if *b== 1 {"is"} else {"are"})]
    DeviceNumberNotCorrect { a: usize, b: usize },
    #[error(
        "Maximum size is {}, but {0} data are to be send",
        crate::cpu::MOD_HEADER_INITIAL_DATA_SIZE
    )]
    ModulationHeadDataSizeOutOfRange(usize),
    #[error(
        "Maximum size is {}, but {0} data are to be send",
        crate::cpu::MOD_HEADER_SUBSEQUENT_DATA_SIZE
    )]
    ModulationBodyDataSizeOutOfRange(usize),
    #[error(
        "Maximum size is {}, but {0} data are to be send",
        crate::cpu::FOCUS_STM_HEAD_DATA_SIZE
    )]
    FocusSTMHeadDataSizeOutOfRange(usize),
    #[error(
        "Maximum size is {}, but {0} data are to be send",
        crate::cpu::FOCUS_STM_BODY_DATA_SIZE
    )]
    FocusSTMBodyDataSizeOutOfRange(usize),
    #[error("PhaseHalf is not supported in Normal mode")]
    PhaseHalfNotSupported,
}
