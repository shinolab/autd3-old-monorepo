/*
 * File: error.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

use autd3_driver::{MAX_CYCLE, NUM_TRANS_IN_UNIT};

#[derive(Error, Debug)]
pub enum AUTDInternalError {
    #[error("Link is closed.")]
    LinkClosed,
    #[error("{} device{} connected, but {} {} specified", a, if *a == 1 {" is"} else {"s are"}, b, if *b== 1 {"is"} else {"are"})]
    DeviceNumberNotCorrect { a: usize, b: usize },
    #[error("{} transducer{} specified, but {} is correct", a,if *a == 1 {" is"} else {"s are"}, NUM_TRANS_IN_UNIT)]
    TransducerNumberNotCorrect { a: usize },
    #[error("Maximum cycle is {} , but {0} is specified", MAX_CYCLE)]
    CycleOutOfRange(u16),
}
