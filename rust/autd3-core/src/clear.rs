/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_driver::NullBody;

use crate::{error::AUTDInternalError, geometry::*, sendable::*};

#[derive(Default)]
pub struct Clear {}

impl Clear {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Sendable<T> for Clear {
    type H = autd3_driver::Clear;
    type B = NullBody;

    fn timeout() -> Option<Duration> {
        Some(Duration::from_millis(200))
    }

    fn operation(&mut self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}
