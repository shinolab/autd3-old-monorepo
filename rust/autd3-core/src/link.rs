/*
 * File: link.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

use autd3_driver::{RxDatagram, TxDatagram};

/// Link is a interface to the AUTD device.
pub trait Link: Send {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError>;
    fn close(&mut self) -> Result<(), AUTDInternalError>;
    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError>;
    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError>;
    fn is_open(&self) -> bool;
    fn timeout(&self) -> Duration;
}
