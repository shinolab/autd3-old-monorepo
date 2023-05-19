/*
 * File: dynamic_link.rs
 * Project: link
 * Created Date: 11/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::core::{error::AUTDInternalError, geometry::Geometry, RxDatagram, TxDatagram};

use crate::dynamic_transducer::DynamicTransducer;

use super::Link;

pub struct DynamicLink {
    link_ptr: Box<dyn Link<DynamicTransducer>>,
}

impl DynamicLink {
    pub fn new(link_ptr: Box<dyn Link<DynamicTransducer>>) -> Self {
        Self { link_ptr }
    }
}

impl Link<DynamicTransducer> for DynamicLink {
    fn open(&mut self, geometry: &Geometry<DynamicTransducer>) -> Result<(), AUTDInternalError> {
        self.link_ptr.open(geometry)
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.link_ptr.close()
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        self.link_ptr.send(tx)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        self.link_ptr.receive(rx)
    }

    fn is_open(&self) -> bool {
        self.link_ptr.is_open()
    }

    fn timeout(&self) -> std::time::Duration {
        self.link_ptr.timeout()
    }
}
