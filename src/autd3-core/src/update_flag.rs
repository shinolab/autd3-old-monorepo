/*
 * File: update_flag.rs
 * Project: src
 * Created Date: 09/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{NullBody, NullHeader};

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

#[derive(Default)]
pub struct UpdateFlags {}

impl UpdateFlags {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for UpdateFlags {
    type H = NullHeader;
    type B = NullBody;

    fn operation(&mut self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}

#[deprecated(since = "11.1.0", note = "Use UpdateFlags instead")]
pub use UpdateFlags as UpdateFlag;
