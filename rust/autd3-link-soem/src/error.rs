/*
 * File: error.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SOEMError {
    #[error("No AUTD device was found")]
    NoDeviceFound,
    #[error("No socket connection on {0}")]
    NoSocketConnection(String),
    #[error("The number of slaves you specified: {1}, but found: {0}")]
    SlaveNotFound(u16, u16),
    #[error("One ore more slaves are not responding")]
    NotResponding,
    #[error("One ore more slaves did not reach safe operational state: {0}")]
    NotReachedSafeOp(u16),
}
