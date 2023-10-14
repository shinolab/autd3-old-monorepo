/*
 * File: builder.rs
 * Project: controller
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{
    geometry::{
        AdvancedPhaseTransducer, AdvancedTransducer, Device, Geometry, IntoDevice,
        LegacyTransducer, Transducer,
    },
    link::LinkBuilder,
};

use super::Controller;
use crate::error::AUTDError;

/// Builder for `Controller`
pub struct ControllerBuilder<T: Transducer> {
    devices: Vec<Device<T>>,
}

impl<T: Transducer> Default for ControllerBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transducer> ControllerBuilder<T> {
    pub(crate) fn new() -> ControllerBuilder<T> {
        Self { devices: vec![] }
    }

    /// Add device
    pub fn add_device<D: IntoDevice<T>>(mut self, dev: D) -> Self {
        self.devices.push(dev.into_device(self.devices.len()));
        self
    }

    /// Open controller
    pub fn open_with<B: LinkBuilder<T>>(
        self,
        link_builder: B,
    ) -> Result<Controller<T, B::L>, AUTDError> {
        let geometry = Geometry::<T>::new(self.devices);
        let link = link_builder.open(&geometry)?;
        Controller::open_impl(geometry, link)
    }

    fn convert<T2: Transducer>(self) -> ControllerBuilder<T2> {
        ControllerBuilder {
            devices: self
                .devices
                .iter()
                .map(|dev| {
                    Device::new(
                        dev.idx(),
                        dev.iter()
                            .map(|tr| T2::new(tr.local_idx(), *tr.position(), *tr.rotation()))
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

impl ControllerBuilder<LegacyTransducer> {
    pub fn advanced(self) -> ControllerBuilder<AdvancedTransducer> {
        self.convert()
    }

    pub fn advanced_phase(self) -> ControllerBuilder<AdvancedPhaseTransducer> {
        self.convert()
    }
}

impl ControllerBuilder<AdvancedTransducer> {
    pub fn legacy(self) -> ControllerBuilder<LegacyTransducer> {
        self.convert()
    }

    pub fn advanced_phase(self) -> ControllerBuilder<AdvancedPhaseTransducer> {
        self.convert()
    }
}

impl ControllerBuilder<AdvancedPhaseTransducer> {
    pub fn advanced(self) -> ControllerBuilder<AdvancedTransducer> {
        self.convert()
    }

    pub fn legacy(self) -> ControllerBuilder<LegacyTransducer> {
        self.convert()
    }
}
