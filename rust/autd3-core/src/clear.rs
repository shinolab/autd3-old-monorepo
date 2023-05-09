/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_driver::NullBody;

use crate::{
    sendable::Sendable,
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

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

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(&mut self, _: &Geometry<T>) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::default())
    }
}
