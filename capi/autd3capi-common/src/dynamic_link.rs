/*
 * File: dynamic_link.rs
 * Project: src
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{error::AUTDInternalError, geometry::Geometry, link::LinkBuilder};

use crate::{DynamicTransducer, L};

pub struct DynamicLinkBuilder {
    link_gen: Box<dyn FnOnce(&Geometry<DynamicTransducer>) -> Result<Box<L>, AUTDInternalError>>,
}

impl DynamicLinkBuilder {
    pub fn new<B: LinkBuilder<DynamicTransducer> + 'static>(b: B) -> Self {
        Self {
            link_gen: Box::new(move |geometry| Ok(Box::new(b.open(geometry)?))),
        }
    }
}

impl LinkBuilder<DynamicTransducer> for DynamicLinkBuilder {
    type L = Box<L>;

    fn open(self, geometry: &Geometry<DynamicTransducer>) -> Result<Self::L, AUTDInternalError> {
        (self.link_gen)(geometry)
    }
}
