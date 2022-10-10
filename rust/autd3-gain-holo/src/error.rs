/*
 * File: error.rs
 * Project: src
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HoloError {
    #[error("Failed to solve linear system")]
    SolveFailed,
}
