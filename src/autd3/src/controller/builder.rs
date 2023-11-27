/*
 * File: builder.rs
 * Project: controller
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::geometry::{Device, Geometry, IntoDevice};

use super::Controller;
use crate::error::AUTDError;

/// Builder for `Controller`
pub struct ControllerBuilder {
    devices: Vec<Device>,
}

impl Default for ControllerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerBuilder {
    pub(crate) fn new() -> ControllerBuilder {
        Self { devices: vec![] }
    }

    /// Add device
    pub fn add_device<D: IntoDevice>(mut self, dev: D) -> Self {
        self.devices.push(dev.into_device(self.devices.len()));
        self
    }

    /// Open controller
    #[cfg(not(feature = "sync"))]
    pub async fn open_with<B: autd3_driver::link::LinkBuilder>(
        self,
        link_builder: B,
    ) -> Result<Controller<B::L>, AUTDError> {
        let geometry = Geometry::new(self.devices);
        let link = link_builder.open(&geometry).await?;
        Controller::open_impl(geometry, link).await
    }

    /// Open controller
    #[cfg(feature = "sync")]
    pub fn open_with<B: autd3_driver::link::LinkSyncBuilder>(
        self,
        link_builder: B,
    ) -> Result<Controller<B::L>, AUTDError> {
        let geometry = Geometry::new(self.devices);
        let link = link_builder.open(&geometry)?;
        Controller::open_impl(geometry, link)
    }
}
