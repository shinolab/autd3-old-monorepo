/*
 * File: mod.rs
 * Project: datagram
 * Created Date: 01/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod clear;

pub use clear::Clear;

use std::time::Duration;

use crate::{error::AUTDInternalError, geometry::*, operation::Operation};

/// Datagram to be sent to devices
pub trait Datagram<T: Transducer> {
    type O1: Operation<T>;
    type O2: Operation<T>;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::O1, Self::O2), AUTDInternalError>;

    fn timeout(&self) -> Option<Duration> {
        None
    }
}
