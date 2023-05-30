/*
 * File: update_flag.rs
 * Project: src
 * Created Date: 09/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{NullBody, NullHeader};

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

#[derive(Default)]
pub struct UpdateFlag {}

impl UpdateFlag {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for UpdateFlag {
    type H = NullHeader;
    type B = NullBody;

    fn operation(&mut self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}
