/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::*,
    error::AUTDInternalError,
    geometry::*,
    operation::{ConfigSilencerOp, StopOp},
};

/// Datagram to stop output
#[derive(Default)]
pub struct Stop {}

impl Stop {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for Stop {
    type O1 = ConfigSilencerOp;
    type O2 = StopOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(10), Self::O2::default()))
    }
}
