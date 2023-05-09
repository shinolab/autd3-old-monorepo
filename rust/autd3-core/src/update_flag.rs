/*
 * File: update_flag.rs
 * Project: src
 * Created Date: 09/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{NullBody, NullHeader};

use crate::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    sendable::Sendable,
};

#[derive(Default)]
pub struct UpdateFlag {}

impl UpdateFlag {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Sendable<T> for UpdateFlag {
    type H = NullHeader;
    type B = NullBody;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Default::default())
    }

    fn body_operation(&mut self, _geometry: &Geometry<T>) -> Result<Self::B, AUTDInternalError> {
        Ok(Default::default())
    }
}
