/*
 * File: dynamic_link.rs
 * Project: src
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{error::AUTDInternalError, geometry::Geometry, link::LinkBuilder};

use crate::{DynamicTransducer, L};

#[async_trait::async_trait]
pub trait DynamicLinkBuilder: Send + Sync {
    async fn open_dyn(
        &mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<Box<L>, AUTDInternalError>;
}

pub struct DynamicLinkBuilderWrapper<B: LinkBuilder<DynamicTransducer> + Sync + Send>
where
    B::L: Send + Sync + 'static,
{
    builder: Option<B>,
}

impl<B: LinkBuilder<DynamicTransducer> + Sync + Send> DynamicLinkBuilderWrapper<B>
where
    B::L: Send + Sync + 'static,
{
    pub fn new(builder: B) -> Self {
        Self {
            builder: Some(builder),
        }
    }
}

#[async_trait::async_trait]
impl<B: LinkBuilder<DynamicTransducer> + Sync + Send> DynamicLinkBuilder
    for DynamicLinkBuilderWrapper<B>
where
    B::L: Send + Sync + 'static,
{
    async fn open_dyn(
        &mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<Box<L>, AUTDInternalError> {
        Ok(Box::new(self.builder.take().unwrap().open(geometry).await?))
    }
}

#[async_trait::async_trait]
impl LinkBuilder<DynamicTransducer> for Box<dyn DynamicLinkBuilder> {
    type L = Box<L>;

    async fn open(
        mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<Self::L, AUTDInternalError> {
        self.open_dyn(geometry).await
    }
}
