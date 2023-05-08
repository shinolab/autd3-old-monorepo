/*
 * File: modulation.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::error::AUTDInternalError;

/// Modulation contains the amplitude modulation data.
pub trait Modulation {
    fn calc(&self) -> Result<Vec<f64>, AUTDInternalError>;
}
