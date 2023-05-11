/*
 * File: update_flag.rs
 * Project: src
 * Created Date: 09/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{NullBody, NullHeader};

use crate::{error::AUTDInternalError, geometry::*, sendable::Sendable};

#[derive(Default)]
pub struct UpdateFlag {}

impl UpdateFlag {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(not(feature = "dynamic"))]
impl<T: Transducer> Sendable<T> for UpdateFlag {
    type H = NullHeader;
    type B = NullBody;

    fn operation(self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}

#[cfg(feature = "dynamic")]
impl Sendable for UpdateFlag {
    fn operation(
        &mut self,
        _: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3_driver::Operation>,
            Box<dyn autd3_driver::Operation>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::new(NullHeader::default()),
            Box::new(NullBody::default()),
        ))
    }
}
